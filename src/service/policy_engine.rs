use crate::domain::policy::{NewProxyPolicy, PolicyAction, PolicyCondition, PolicyDecision, ProxyPolicy, RequestContext};
use crate::service::proxy_engine::{ProxyError, ProxyService};
use async_trait::async_trait;
use chrono::NaiveTime;
use ipnet::IpNet;
use regex::RegexBuilder;
use sqlx::{PgPool, Row};
use std::{net::IpAddr, str::FromStr, sync::Arc};
use uuid::Uuid;

type Result<T> = std::result::Result<T, ProxyError>;
const MAX_CONDITIONS: usize = 24;
const MAX_REGEX_LEN: usize = 256;

#[async_trait]
pub trait PolicyEvaluator: Send + Sync {
    async fn evaluate(&self, context: &RequestContext) -> Result<PolicyDecision>;
    async fn list(&self) -> Result<Vec<ProxyPolicy>>;
    async fn create(&self, actor: &str, policy: NewProxyPolicy) -> Result<ProxyPolicy>;
    async fn delete(&self, actor: &str, id: Uuid) -> Result<()>;
}

pub struct PostgresPolicyEvaluator {
    db: PgPool,
    reputation: Option<crate::service::feed_manager::FeedManagerRef>,
}

impl PostgresPolicyEvaluator {
    pub fn new(db: PgPool, reputation: Option<crate::service::feed_manager::FeedManagerRef>) -> Self { Self {db,reputation} }

    fn validate_condition(c: &PolicyCondition) -> Result<()> {
        match c {
            PolicyCondition::SourceIpRange(v) | PolicyCondition::DestinationIpRange(v) => {
                IpNet::from_str(v).map_err(|_|ProxyError("invalid CIDR".into()))?;
            },
            PolicyCondition::UrlRegex(v) | PolicyCondition::UserAgentRegex(v) => validate_regex(v)?,
            PolicyCondition::Header{name,value_regex} => {
                if name.is_empty() || !name.bytes().all(|b|b.is_ascii_alphanumeric() || b==b'-') {
                    return Err(ProxyError("invalid header name".into()));
                }
                validate_regex(value_regex)?;
            },
            PolicyCondition::TimeWindow{start_utc,end_utc}=>{
                NaiveTime::parse_from_str(start_utc,"%H:%M").map_err(|_|ProxyError("invalid UTC time".into()))?;
                NaiveTime::parse_from_str(end_utc,"%H:%M").map_err(|_|ProxyError("invalid UTC time".into()))?;
            },
            PolicyCondition::DestinationDomain(v) => {
                if v.len()>253 || v.is_empty() || v.contains('/') || v.contains(' ') { return Err(ProxyError("invalid domain".into())); }
            },
            PolicyCondition::HttpMethod(v) => {
                if !["GET","POST","PUT","PATCH","DELETE","HEAD","OPTIONS","CONNECT"].contains(&v.as_str()) {return Err(ProxyError("invalid method".into()));}
            },
            PolicyCondition::AdGroup(v) | PolicyCondition::WebCategory(v) | PolicyCondition::Application(v)
            | PolicyCondition::Interface(v) | PolicyCondition::Zone(v) if v.is_empty() || v.len()>128 => {
                return Err(ProxyError("invalid condition value".into()));
            },
            _=>{}
        }
        Ok(())
    }

    fn matches(condition:&PolicyCondition,c:&RequestContext)->bool{
        match condition {
            PolicyCondition::SourceIpRange(cidr)=>cidr.parse::<IpNet>().is_ok_and(|n|n.contains(&c.source_ip)),
            PolicyCondition::DestinationIpRange(cidr)=>c.destination_ip.is_some_and(|ip|cidr.parse::<IpNet>().is_ok_and(|n|n.contains(&ip))),
            PolicyCondition::DestinationDomain(pattern)=>{
                let p=pattern.to_ascii_lowercase();
                let h=c.host.trim_end_matches('.').to_ascii_lowercase();
                h==p || p.strip_prefix("*.").is_some_and(|s|h.ends_with(&format!(".{s}")))
            },
            PolicyCondition::UrlRegex(v)=>safe_regex(v,&c.url),
            PolicyCondition::TimeWindow{start_utc,end_utc}=>{
                let Ok(start)=NaiveTime::parse_from_str(start_utc,"%H:%M") else {return false};
                let Ok(end)=NaiveTime::parse_from_str(end_utc,"%H:%M") else {return false};
                let time=c.timestamp.time();
                if start<=end {time>=start&&time<end} else {time>=start||time<end}
            },
            PolicyCondition::HttpMethod(v)=>c.method.eq_ignore_ascii_case(v),
            PolicyCondition::DestinationPort(v)=>c.port==*v,
            PolicyCondition::UserAgentRegex(v)=>c.user_agent.as_ref().is_some_and(|u|safe_regex(v,u)),
            PolicyCondition::Header{name,value_regex}=>c.headers.iter().any(|(k,v)|k.eq_ignore_ascii_case(name)&&safe_regex(value_regex,v)),
            PolicyCondition::AdGroup(v)=>c.groups.iter().any(|g|g==v),
            PolicyCondition::WebCategory(v)=>c.category.as_ref().is_some_and(|s|s==v),
            PolicyCondition::Application(v)=>c.application.as_ref().is_some_and(|s|s==v),
            PolicyCondition::Interface(v)=>c.interface.as_ref().is_some_and(|s|s==v),
            PolicyCondition::Zone(v)=>c.zone.as_ref().is_some_and(|s|s==v),
        }
    }

    async fn audit_tx(&self, actor:&str,operation:&str,target:&str,details:serde_json::Value,
        tx:&mut sqlx::Transaction<'_,sqlx::Postgres>)->Result<()> {
        sqlx::query("INSERT INTO proxy_audit_logs(actor,identity_source,operation,target,outcome,details) VALUES($1,'verified',$2,$3,'ALLOW',$4)")
            .bind(actor).bind(operation).bind(target).bind(details).execute(&mut **tx).await
            .map_err(|e|ProxyError(e.to_string()))?;
        Ok(())
    }
}

fn validate_regex(value:&str)->Result<()> {
    if value.is_empty() || value.len()>MAX_REGEX_LEN {return Err(ProxyError("regex length invalid".into()));}
    RegexBuilder::new(value).size_limit(512*1024).dfa_size_limit(512*1024).build()
        .map_err(|_|ProxyError("invalid regex".into()))?;
    Ok(())
}
fn safe_regex(pattern:&str,input:&str)->bool {
    if input.len()>8192 {return false;}
    RegexBuilder::new(pattern).size_limit(512*1024).dfa_size_limit(512*1024).build()
        .is_ok_and(|r|r.is_match(input))
}

#[async_trait]
impl PolicyEvaluator for PostgresPolicyEvaluator {
    async fn list(&self)->Result<Vec<ProxyPolicy>> {
        let rows=sqlx::query("SELECT id,name,action,conditions,priority,enabled FROM proxy_policies ORDER BY priority DESC,id ASC")
            .fetch_all(&self.db).await.map_err(|e|ProxyError(e.to_string()))?;
        rows.into_iter().map(|r|{
            let action: String=r.try_get("action").map_err(|e|ProxyError(e.to_string()))?;
            let conditions:serde_json::Value=r.try_get("conditions").map_err(|e|ProxyError(e.to_string()))?;
            let conditions=serde_json::from_value(conditions).map_err(|e|ProxyError(e.to_string()))?;
            let action=match action.as_str() {"ALLOW"=>PolicyAction::Allow,"DENY"=>PolicyAction::Deny,"INSPECT"=>PolicyAction::Inspect,_=>return Err(ProxyError("invalid stored action".into()))};
            Ok(ProxyPolicy{id:r.try_get("id").map_err(|e|ProxyError(e.to_string()))?,
                name:r.try_get("name").map_err(|e|ProxyError(e.to_string()))?, action,conditions,
                priority:r.try_get("priority").map_err(|e|ProxyError(e.to_string()))?,
                enabled:r.try_get("enabled").map_err(|e|ProxyError(e.to_string()))?})
        }).collect()
    }
    async fn evaluate(&self,c:&RequestContext)->Result<PolicyDecision>{
        let mut context=c.clone();
        context.category=None;
        if let Some(manager)=&self.reputation {
            context.category=manager.resolve(&context.host).await.map_err(ProxyError)?.map(|r|r.category.slug);
        }
        let c=&context;
        // A failed DB fetch must NEVER turn into an allow decision.
        for p in self.list().await? {
            if p.enabled && !p.conditions.is_empty() && p.conditions.iter().all(|v|Self::matches(v,c)){
                let decision=PolicyDecision{action:if p.action==PolicyAction::Inspect {PolicyAction::Deny} else {p.action},policy_id:Some(p.id),reason:if p.action==PolicyAction::Inspect {"inspection unavailable: fail closed".into()} else {"first matching policy".into()}};
                self.record_decision(c,&decision).await?;
                return Ok(decision)
            }
        }
        let decision=PolicyDecision{action:PolicyAction::Deny,policy_id:None,reason:"default deny".into()};
        self.record_decision(c,&decision).await?;
        Ok(decision)
    }
    async fn create(&self,actor:&str,p:NewProxyPolicy)->Result<ProxyPolicy>{
        if p.name.trim().is_empty() || p.name.len()>160 || p.conditions.is_empty() || p.conditions.len()>MAX_CONDITIONS {
            return Err(ProxyError("invalid policy".into()));
        }
        for c in &p.conditions {Self::validate_condition(c)?;}
        let action=match p.action {PolicyAction::Allow=>"ALLOW",PolicyAction::Deny=>"DENY",PolicyAction::Inspect=>"INSPECT"};
        let value=serde_json::to_value(&p.conditions).map_err(|e|ProxyError(e.to_string()))?;
        let mut tx=self.db.begin().await.map_err(|e|ProxyError(e.to_string()))?;
        let row=sqlx::query("INSERT INTO proxy_policies(name,action,conditions,priority,created_by) VALUES($1,$2,$3,$4,$5) RETURNING id")
            .bind(&p.name).bind(action).bind(value).bind(p.priority).bind(actor)
            .fetch_one(&mut *tx).await.map_err(|e|ProxyError(e.to_string()))?;
        let id:Uuid=row.try_get("id").map_err(|e|ProxyError(e.to_string()))?;
        self.audit_tx(actor,"policy_create",&id.to_string(),serde_json::json!({"name":p.name,"action":action}),&mut tx).await?;
        tx.commit().await.map_err(|e|ProxyError(e.to_string()))?;
        Ok(ProxyPolicy{id,name:p.name,action:p.action,conditions:p.conditions,priority:p.priority,enabled:true})
    }
    async fn delete(&self,actor:&str,id:Uuid)->Result<()>{
        let mut tx=self.db.begin().await.map_err(|e|ProxyError(e.to_string()))?;
        let r=sqlx::query("DELETE FROM proxy_policies WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(|e|ProxyError(e.to_string()))?;
        if r.rows_affected()==0{return Err(ProxyError("not found".into()));}
        self.audit_tx(actor,"policy_delete",&id.to_string(),serde_json::json!({}),&mut tx).await?;
        tx.commit().await.map_err(|e|ProxyError(e.to_string()))
    }
}

impl PostgresPolicyEvaluator {
    async fn record_decision(&self,c:&RequestContext,d:&PolicyDecision)->Result<()>{
        let action=match d.action{PolicyAction::Allow=>"ALLOW",PolicyAction::Deny=>"DENY",PolicyAction::Inspect=>"ALLOW"};
        sqlx::query("INSERT INTO proxy_audit_logs(actor,identity_source,operation,target,outcome,details) VALUES($1,$2,'policy_decision',$3,$4,$5)")
            .bind(&c.identity_id).bind(&c.provider).bind(format!("{}:{}",c.host,c.port)).bind(action)
            .bind(serde_json::json!({"policy_id":d.policy_id,"policy_action":d.action,"url":c.url,"source_ip":c.source_ip,"destination_ip":c.destination_ip,"reason":d.reason}))
            .execute(&self.db).await.map_err(|e|ProxyError(e.to_string()))?;
        tracing::info!(identity_id=%c.identity_id, host=%c.host, ?d.action, "policy_decision");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;
    #[test]
    fn domain_suffix_boundary(){
        let ctx=RequestContext{identity_id:"test".into(),provider:"AD".into(),groups:vec![],source_ip:"10.1.1.1".parse().unwrap(),
            destination_ip:None,host:"badexample.com".into(),url:"https://badexample.com".into(),method:"GET".into(),port:443,
            user_agent:None,headers:HashMap::new(),category:None,application:None,interface:None,zone:None,timestamp:Utc::now()};
        assert!(!PostgresPolicyEvaluator::matches(&PolicyCondition::DestinationDomain("*.example.com".into()),&ctx));
    }
}
