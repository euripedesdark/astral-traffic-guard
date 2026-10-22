use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AclRule {
    pub id: String,
    pub name: String,
    pub pattern: String,
    pub category: String,
    pub action: String, // "ALLOW", "DENY"
    pub priority: i32,
    pub user_group: Option<String>,
    pub enabled: bool,
    pub created_at: String,
}

impl AclRule {
    pub fn matches_domain(&self, domain: &str) -> bool {
        let pattern = self.pattern.trim().to_lowercase();
        let target = domain.trim().to_lowercase();

        if pattern == "*" || pattern == target {
            return true;
        }

        if let Some(suffix) = pattern.strip_prefix("*.") {
            return target.ends_with(suffix) || target == suffix;
        }

        if let Some(prefix) = pattern.strip_suffix(".*") {
            return target.starts_with(prefix);
        }

        target.contains(&pattern)
    }

    pub fn matches_group(&self, group: Option<&str>) -> bool {
        match (&self.user_group, group) {
            (None, _) => true,
            (Some(rule_grp), Some(user_grp)) => {
                rule_grp == "*" || rule_grp.eq_ignore_ascii_case(user_grp)
            }
            (Some(rule_grp), None) => rule_grp == "*",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AclCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default_action: String,
    pub domains: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckRequest {
    pub domain: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub ip: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckResponse {
    pub allowed: bool,
    pub action: String,
    pub matched_rule: Option<String>,
    pub reason: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProxyStatus {
    pub active: bool,
    pub uptime_seconds: u64,
    pub total_requests: usize,
    pub blocked_requests: usize,
    pub rules_count: usize,
    pub categories_count: usize,
}
