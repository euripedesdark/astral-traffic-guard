use async_trait::async_trait;
use crate::domain::proxy::{DomainReputation, ReputationSource, WebCategory};
use moka::future::Cache;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool,Row};
use std::{net::IpAddr, sync::Arc, time::Duration};
use uuid::Uuid;

type Result<T> = std::result::Result<T,String>;
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Feed {pub id:Uuid,pub name:String,pub url:String,pub category:String,pub enabled:bool}
#[derive(Debug,Deserialize)]
pub struct NewFeed {pub name:String,pub url:String,pub category:String}
#[derive(Debug,Serialize)]
pub struct RefreshReport {pub feed_id:Uuid,pub entries:usize}
#[async_trait]
pub trait ReputationService:Send+Sync {
 async fn resolve(&self,host:&str)->Result<Option<DomainReputation>>;
 async fn feeds(&self)->Result<Vec<Feed>>;
 async fn add(&self,feed:NewFeed)->Result<Feed>;
 async fn refresh(&self,id:Uuid)->Result<RefreshReport>;
 async fn categories(&self)->Result<Vec<WebCategory>>;
}
pub struct PostgresFeedManager{
 db:PgPool,
 http:Client,
 cache:Cache<String,Option<DomainReputation>>,
 allowed_hosts:Vec<String>,
}
impl PostgresFeedManager{
 pub fn new(db:PgPool,allowed_hosts:Vec<String>)->Result<Self>{
  if allowed_hosts.is_empty(){return Err("ASTRAL_FEED_ALLOWED_HOSTS must be configured".into());}
  let http=Client::builder().redirect(reqwest::redirect::Policy::none())
   .timeout(Duration::from_secs(20)).build().map_err(|e|e.to_string())?;
  Ok(Self{db,http,cache:Cache::builder().max_capacity(20_000).time_to_live(Duration::from_secs(120)).build(),allowed_hosts})
 }
 fn checked_url(&self,url:&str)->Result<Url>{
  let url=Url::parse(url).map_err(|_|"invalid feed URL".to_string())?;
  if url.scheme()!="https" || !url.username().is_empty() || url.password().is_some() || url.port().is_some(){
   return Err("feeds require plain HTTPS without credentials or custom ports".into());
  }
  let host=url.host_str().ok_or("missing host")?;
  if host.parse::<IpAddr>().is_ok() || !self.allowed_hosts.iter().any(|h|h.eq_ignore_ascii_case(host)){
   return Err("feed hostname not allowlisted".into());
  }
  Ok(url)
 }
 fn normalize(value:&str)->Option<(String,String)>{
  let x=value.trim().trim_matches('"').trim_end_matches('.').to_ascii_lowercase();
  if let Ok(ip)=x.parse::<IpAddr>(){return Some((ip.to_string(),"ip".into()));}
  if x.is_empty() || x.len()>253 || x.contains("..") || !x.contains('.') ||
   !x.split('.').all(|part|!part.is_empty()&&part.len()<=63&&
    !part.starts_with('-')&&!part.ends_with('-')&&part.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'-')){
   return None;
  }
  Some((x,"domain".into()))
 }
 fn parse_feed(body:&str)->Result<Vec<(String,String)>>{
  if body.len()>8_000_000{return Err("feed exceeds 8MB".into());}
  let mut entries=std::collections::BTreeSet::new();
  for line in body.lines(){
   let line=line.trim();
   if line.is_empty()||line.starts_with('#')||line.starts_with(';'){continue;}
   let raw=line.split(',').next().unwrap_or("").trim();
   if let Some(entry)=Self::normalize(raw){entries.insert(entry);}
   if entries.len()>100_000{return Err("feed exceeds 100,000 entries".into());}
  }
  if entries.is_empty(){return Err("refusing empty feed update".into());}
  Ok(entries.into_iter().collect())
 }
}
#[async_trait]
impl ReputationService for PostgresFeedManager{
 async fn categories(&self)->Result<Vec<WebCategory>>{
  let values=sqlx::query_scalar::<_,String>("SELECT slug FROM web_categories ORDER BY slug").fetch_all(&self.db).await.map_err(|e|e.to_string())?;
  Ok(values.into_iter().map(|slug|WebCategory{slug}).collect())
 }
 async fn feeds(&self)->Result<Vec<Feed>>{
  sqlx::query_as::<_,(Uuid,String,String,String,bool)>("SELECT id,name,url,category,enabled FROM reputation_feeds ORDER BY name")
   .fetch_all(&self.db).await.map_err(|e|e.to_string()).map(|v|v.into_iter().map(|(id,name,url,category,enabled)|Feed{id,name,url,category,enabled}).collect())
 }
 async fn add(&self,feed:NewFeed)->Result<Feed>{
  self.checked_url(&feed.url)?;
  if feed.name.trim().is_empty()||feed.name.len()>120{return Err("invalid name".into());}
  let (id,name,url,category,enabled)=sqlx::query_as::<_,(Uuid,String,String,String,bool)>(
   "INSERT INTO reputation_feeds(name,url,category) VALUES($1,$2,$3) RETURNING id,name,url,category,enabled")
   .bind(&feed.name).bind(&feed.url).bind(&feed.category).fetch_one(&self.db).await.map_err(|e|e.to_string())?;
  Ok(Feed{id,name,url,category,enabled})
 }
 async fn refresh(&self,id:Uuid)->Result<RefreshReport>{
  let feed=sqlx::query_as::<_,(String,String,bool)>("SELECT url,category,enabled FROM reputation_feeds WHERE id=$1")
   .bind(id).fetch_one(&self.db).await.map_err(|e|e.to_string())?;
  if !feed.2{return Err("feed disabled".into());}
  let url=self.checked_url(&feed.0)?;
  let res=self.http.get(url).send().await.map_err(|e|e.to_string())?;
  if !res.status().is_success(){return Err(format!("feed upstream status {}",res.status()));}
  if res.content_length().is_some_and(|n|n>8_000_000){return Err("feed exceeds size limit".into());}
  let bytes=res.bytes().await.map_err(|e|e.to_string())?;
  let body=std::str::from_utf8(&bytes).map_err(|_|"feed must be UTF-8")?;
  let entries=Self::parse_feed(body)?;
  let mut tx=self.db.begin().await.map_err(|e|e.to_string())?;
  sqlx::query("DELETE FROM domain_reputation WHERE feed_id=$1").bind(id).execute(&mut *tx).await.map_err(|e|e.to_string())?;
  for (value,kind) in &entries{
   sqlx::query("INSERT INTO domain_reputation(indicator,kind,category,source,feed_id) VALUES($1,$2,$3,'external_feed',$4)")
    .bind(value).bind(kind).bind(&feed.1).bind(id).execute(&mut *tx).await.map_err(|e|e.to_string())?;
  }
  sqlx::query("UPDATE reputation_feeds SET last_updated=now(),last_count=$2 WHERE id=$1")
   .bind(id).bind(entries.len() as i32).execute(&mut *tx).await.map_err(|e|e.to_string())?;
  tx.commit().await.map_err(|e|e.to_string())?;
  self.cache.invalidate_all();
  tracing::info!(feed_id=%id,count=entries.len(),"feed refreshed");
  Ok(RefreshReport{feed_id:id,entries:entries.len()})
 }
 async fn resolve(&self,host:&str)->Result<Option<DomainReputation>>{
  let (normalized,kind)=Self::normalize(host).ok_or("invalid destination")?;
  if let Some(hit)=self.cache.get(&normalized).await{return Ok(hit);}
  // Match exact indicators, then suffixes at actual DNS label boundaries.
  let mut candidates=vec![normalized.clone()];
  if kind=="domain"{
   let mut part=normalized.as_str();
   while let Some((_,remainder))=part.split_once('.'){
    if !remainder.contains('.'){break;}
    candidates.push(remainder.to_string());part=remainder;
   }
  }
  let item=sqlx::query("SELECT category,source FROM domain_reputation WHERE indicator=ANY($1) ORDER BY CASE category WHEN 'malware' THEN 0 WHEN 'phishing' THEN 1 ELSE 2 END,indicator DESC LIMIT 1")
   .bind(&candidates).fetch_optional(&self.db).await.map_err(|e|e.to_string())?;
  let result=item.map(|row|{
   let category:String=row.get("category");let source:String=row.get("source");
   DomainReputation{domain:normalized.clone(),category:WebCategory{slug:category},source:match source.as_str(){"manual"=>ReputationSource::Manual,"ai_analysis"=>ReputationSource::AiAnalysis,_=>ReputationSource::ExternalFeed}}
  });
  self.cache.insert(normalized,result.clone()).await;
  Ok(result)
 }
}
pub type FeedManagerRef=Arc<dyn ReputationService>;
