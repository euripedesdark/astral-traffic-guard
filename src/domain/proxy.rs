use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentitySource { Ad, Local }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityContext {
    pub source: IdentitySource,
    pub identity_id: String,
    pub username: String,
    pub groups: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProxyRule {
    pub id: Uuid,
    pub name: String,
    pub identity_source: String,
    pub identity_group: Option<String>,
    pub host_pattern: String,
    pub port: Option<i32>,
    pub action: String,
    pub priority: i32,
    pub enabled: bool,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewProxyRule {
    pub name: String,
    pub identity_source: String,
    pub identity_group: Option<String>,
    pub host_pattern: String,
    pub port: Option<i32>,
    pub action: String,
    pub priority: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProxyAuditLog {
    pub id: Uuid,
    pub recorded_at: DateTime<Utc>,
    pub actor: String,
    pub identity_source: String,
    pub operation: String,
    pub target: String,
    pub outcome: String,
    pub details: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DecisionRequest {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Serialize)]
pub struct Decision {
    pub allowed: bool,
    pub reason: String,
    pub matched_rule_id: Option<Uuid>,
}
