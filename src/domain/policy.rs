use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::net::IpAddr;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicyAction { Allow, Deny, Inspect }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag="type", content="value", rename_all="snake_case")]
pub enum PolicyCondition {
    SourceIpRange(String),
    DestinationIpRange(String),
    DestinationDomain(String),
    UrlRegex(String),
    TimeWindow { start_utc: String, end_utc: String },
    HttpMethod(String),
    DestinationPort(u16),
    UserAgentRegex(String),
    Header { name: String, value_regex: String },
    AdGroup(String),
    WebCategory(String),
    Application(String),
    Interface(String),
    Zone(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProxyPolicy {
    pub id: Uuid,
    pub name: String,
    pub action: PolicyAction,
    pub conditions: Vec<PolicyCondition>,
    pub priority: i32,
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewProxyPolicy {
    pub name: String,
    pub action: PolicyAction,
    pub conditions: Vec<PolicyCondition>,
    pub priority: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestContext {
    pub identity_id: String,
    pub provider: String,
    pub groups: Vec<String>,
    pub source_ip: IpAddr,
    pub destination_ip: Option<IpAddr>,
    pub host: String,
    pub url: String,
    pub method: String,
    pub port: u16,
    pub user_agent: Option<String>,
    pub headers: std::collections::HashMap<String,String>,
    pub category: Option<String>,
    pub application: Option<String>,
    pub interface: Option<String>,
    pub zone: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PolicyDecision {
    pub action: PolicyAction,
    pub policy_id: Option<Uuid>,
    pub reason: String,
}
