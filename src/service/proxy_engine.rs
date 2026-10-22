use crate::domain::proxy::{Decision, DecisionRequest, IdentityContext, IdentitySource, NewProxyRule, ProxyAuditLog, ProxyRule};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug)]
pub struct ProxyError(pub String);
impl std::fmt::Display for ProxyError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
impl std::error::Error for ProxyError {}
type Result<T> = std::result::Result<T, ProxyError>;

#[async_trait]
pub trait ProxyService: Send + Sync {
    async fn identity(&self, bearer: &str) -> Result<IdentityContext>;
    async fn rules(&self) -> Result<Vec<ProxyRule>>;
    async fn create_rule(&self, actor: &IdentityContext, rule: NewProxyRule) -> Result<ProxyRule>;
    async fn delete_rule(&self, actor: &IdentityContext, id: Uuid) -> Result<()>;
    async fn audit(&self, limit: i64) -> Result<Vec<ProxyAuditLog>>;
    async fn decide(&self, actor: &IdentityContext, request: DecisionRequest) -> Result<Decision>;
}

#[derive(Deserialize)]
struct AuthReply {
    active: bool,
    provider: Option<String>,
    identity_id: Option<String>,
    username: Option<String>,
    #[serde(default)] groups: Vec<String>,
    #[serde(default)] roles: Vec<String>,
}

pub struct PostgresProxyService {
    db: PgPool,
    http: Client,
    auth_url: String,
    auth_client_token: String,
}
impl PostgresProxyService {
    pub fn new(db: PgPool, auth_url: String, auth_client_token: String) -> Result<Self> {
        if !auth_url.starts_with("https://") || auth_client_token.is_empty() {
            return Err(ProxyError("AUTH_SERVICE_INTROSPECT_URL must be HTTPS and AUTH_SERVICE_CLIENT_TOKEN must be set".into()));
        }
        let http = Client::builder().timeout(Duration::from_secs(4)).build().map_err(|e| ProxyError(e.to_string()))?;
        Ok(Self { db, http, auth_url, auth_client_token })
    }
    async fn event(&self, actor: &IdentityContext, operation: &str, target: &str, outcome: &str, details: serde_json::Value) -> Result<()> {
        sqlx::query("INSERT INTO proxy_audit_logs (actor, identity_source, operation, target, outcome, details) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(&actor.identity_id).bind(match actor.source { IdentitySource::Ad => "ad", IdentitySource::Local => "local" })
            .bind(operation).bind(target).bind(outcome).bind(details)
            .execute(&self.db).await.map_err(|e| ProxyError(e.to_string()))?;
        tracing::info!(identity_id = %actor.identity_id, operation, target, outcome, "proxy_audit");
        Ok(())
    }
    fn management(actor: &IdentityContext) -> Result<()> {
        if actor.roles.iter().any(|r| r == "proxy_admin") { Ok(()) } else { Err(ProxyError("forbidden".into())) }
    }
}
#[async_trait]
impl ProxyService for PostgresProxyService {
    async fn identity(&self, bearer: &str) -> Result<IdentityContext> {
        if bearer.len() > 8192 || bearer.is_empty() { return Err(ProxyError("unauthorized".into())); }
        // Contract: auth-service introspection must authenticate this client, validate signature,
        // issuer, audience, expiry, revocation, and return active=false on any failure.
        let res = self.http.post(&self.auth_url)
            .bearer_auth(&self.auth_client_token)
            .json(&json!({"token":bearer}))
            .send().await.map_err(|_| ProxyError("authentication unavailable".into()))?;
        if !res.status().is_success() { return Err(ProxyError("unauthorized".into())); }
        let r: AuthReply = res.json().await.map_err(|_| ProxyError("unauthorized".into()))?;
        if !r.active { return Err(ProxyError("unauthorized".into())); }
        let (Some(provider),Some(identity_id),Some(username)) = (r.provider, r.identity_id, r.username) else { return Err(ProxyError("unauthorized".into())); };
        if identity_id.is_empty() || username.is_empty() { return Err(ProxyError("unauthorized".into())); }
        let source = match provider.as_str() { "AD" | "ad" => IdentitySource::Ad, "POSTGRESQL" | "local" => IdentitySource::Local, _ => return Err(ProxyError("unauthorized".into())) };
        // Local identities are issued only by auth-service; PostgreSQL database role is not a user password store.
        if matches!(source, IdentitySource::Local) && !r.roles.iter().any(|v| v == "proxy" || v == "proxy_admin") {
            return Err(ProxyError("unauthorized".into()));
        }
        Ok(IdentityContext { source, identity_id, username, groups:r.groups, roles:r.roles })
    }
    async fn rules(&self) -> Result<Vec<ProxyRule>> {
        sqlx::query_as::<_, ProxyRule>("SELECT id,name,identity_source,identity_group,host_pattern,port,action,priority,enabled,created_by,created_at FROM proxy_rules ORDER BY priority DESC,id")
            .fetch_all(&self.db).await.map_err(|e| ProxyError(e.to_string()))
    }
    async fn create_rule(&self, actor: &IdentityContext, r: NewProxyRule) -> Result<ProxyRule> {
        Self::management(actor)?;
        if r.name.trim().is_empty() || !["ad","local","any"].contains(&r.identity_source.as_str())
            || !["ALLOW","DENY"].contains(&r.action.as_str()) || !valid_pattern(&r.host_pattern)
            || r.port.map_or(false,|p| !(1..=65535).contains(&p)) {
            return Err(ProxyError("invalid proxy rule".into()));
        }
        // Audit failure MUST abort rule change: one database transaction.
        let mut tx=self.db.begin().await.map_err(|e| ProxyError(e.to_string()))?;
        let row=sqlx::query_as::<_,ProxyRule>("INSERT INTO proxy_rules(name,identity_source,identity_group,host_pattern,port,action,priority,created_by) VALUES($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id,name,identity_source,identity_group,host_pattern,port,action,priority,enabled,created_by,created_at")
            .bind(&r.name).bind(&r.identity_source).bind(&r.identity_group).bind(&r.host_pattern).bind(r.port).bind(&r.action).bind(r.priority).bind(&actor.identity_id)
            .fetch_one(&mut *tx).await.map_err(|e| ProxyError(e.to_string()))?;
        sqlx::query("INSERT INTO proxy_audit_logs(actor,identity_source,operation,target,outcome,details) VALUES($1,$2,'rule_create',$3,'ALLOW',$4)")
            .bind(&actor.identity_id).bind(match actor.source {IdentitySource::Ad=>"ad",IdentitySource::Local=>"local"})
            .bind(row.id.to_string()).bind(json!({"name":r.name,"action":r.action}))
            .execute(&mut *tx).await.map_err(|e|ProxyError(e.to_string()))?;
        tx.commit().await.map_err(|e|ProxyError(e.to_string()))?;
        Ok(row)
    }
    async fn delete_rule(&self, actor: &IdentityContext, id: Uuid) -> Result<()> {
        Self::management(actor)?;
        let mut tx=self.db.begin().await.map_err(|e|ProxyError(e.to_string()))?;
        let deleted=sqlx::query("DELETE FROM proxy_rules WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(|e|ProxyError(e.to_string()))?;
        if deleted.rows_affected()==0 {return Err(ProxyError("not found".into()));}
        sqlx::query("INSERT INTO proxy_audit_logs(actor,identity_source,operation,target,outcome) VALUES($1,$2,'rule_delete',$3,'ALLOW')")
            .bind(&actor.identity_id).bind(match actor.source {IdentitySource::Ad=>"ad",IdentitySource::Local=>"local"}).bind(id.to_string())
            .execute(&mut *tx).await.map_err(|e|ProxyError(e.to_string()))?;
        tx.commit().await.map_err(|e|ProxyError(e.to_string()))
    }
    async fn audit(&self, limit: i64) -> Result<Vec<ProxyAuditLog>> {
        sqlx::query_as::<_, ProxyAuditLog>("SELECT id,recorded_at,actor,identity_source,operation,target,outcome,details FROM proxy_audit_logs ORDER BY recorded_at DESC,id DESC LIMIT $1")
            .bind(limit.clamp(1,500)).fetch_all(&self.db).await.map_err(|e|ProxyError(e.to_string()))
    }
    async fn decide(&self, actor: &IdentityContext, request: DecisionRequest) -> Result<Decision> {
        if !valid_host(&request.host) || request.port==0 { return Err(ProxyError("invalid target".into())); }
        let rules=self.rules().await?;
        let src=match actor.source { IdentitySource::Ad=>"ad",IdentitySource::Local=>"local" };
        let mut decision=Decision { allowed:false, reason:"default deny".into(), matched_rule_id:None };
        for rule in rules {
            if !rule.enabled || !(rule.identity_source=="any" || rule.identity_source==src) {continue;}
            if rule.port.map_or(false,|p|p != request.port as i32) {continue;}
            if rule.identity_group.as_ref().map_or(false,|g|!actor.groups.contains(g)) {continue;}
            if !match_host(&rule.host_pattern,&request.host) {continue;}
            decision=Decision {allowed:rule.action=="ALLOW", reason:rule.action, matched_rule_id:Some(rule.id)};
            break;
        }
        self.event(actor,"proxy_decision",&format!("{}:{}",request.host,request.port),
            if decision.allowed {"ALLOW"} else {"DENY"},json!({"rule":decision.matched_rule_id})).await?;
        Ok(decision)
    }
}
fn valid_host(host:&str)->bool { !host.is_empty() && host.len()<=253 && host.bytes().all(|b|b.is_ascii_alphanumeric() || b==b'.' || b==b'-') && !host.contains("..") }
fn valid_pattern(pattern:&str)->bool { pattern=="*" || valid_host(pattern) || pattern.strip_prefix("*.").map_or(false,valid_host) }
fn match_host(pattern:&str,host:&str)->bool {
    let host=host.to_ascii_lowercase();let p=pattern.to_ascii_lowercase();
    if p=="*" || p==host {return true;}
    if let Some(suffix)=p.strip_prefix("*."){return host.ends_with(&format!(".{}",suffix));}
    false
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn wildcard_no_suffix_confusion(){ assert!(match_host("*.example.com","a.example.com")); assert!(!match_host("*.example.com","badexample.com")); assert!(!match_host("*.example.com","example.com")); }
}
