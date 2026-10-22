use crate::domain::{AclCategory, AclRule, CheckRequest, CheckResponse, ProxyStatus, TrafficGuardError};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing::info;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
struct PersistedData {
    rules: Vec<AclRule>,
    categories: Vec<AclCategory>,
}

pub trait AclEngine: Send + Sync {
    fn get_rules(&self) -> Vec<AclRule>;
    fn add_rule(&self, rule: AclRule) -> Result<AclRule, TrafficGuardError>;
    fn delete_rule(&self, id: &str) -> Result<bool, TrafficGuardError>;
    fn get_categories(&self) -> Vec<AclCategory>;
    fn evaluate(&self, req: &CheckRequest) -> CheckResponse;
    fn get_status(&self) -> ProxyStatus;
}

pub struct StatefulAclEngine {
    storage_path: PathBuf,
    rules: Mutex<Vec<AclRule>>,
    categories: Mutex<Vec<AclCategory>>,
    start_time: Instant,
    total_requests: Mutex<usize>,
    blocked_requests: Mutex<usize>,
}

impl StatefulAclEngine {
    pub fn new(storage_path: PathBuf) -> Self {
        let (rules, categories) = Self::load_from_disk(&storage_path);
        Self {
            storage_path,
            rules: Mutex::new(rules),
            categories: Mutex::new(categories),
            start_time: Instant::now(),
            total_requests: Mutex::new(0),
            blocked_requests: Mutex::new(0),
        }
    }

    fn default_categories() -> Vec<AclCategory> {
        vec![
            AclCategory {
                id: "social".to_string(),
                name: "Redes Sociais".to_string(),
                description: "Facebook, Instagram, TikTok, Twitter/X".to_string(),
                default_action: "DENY".to_string(),
                domains: vec!["facebook.com".into(), "instagram.com".into(), "tiktok.com".into(), "twitter.com".into(), "x.com".into()],
            },
            AclCategory {
                id: "streaming".to_string(),
                name: "Streaming & Vídeo".to_string(),
                description: "YouTube, Netflix, Twitch".to_string(),
                default_action: "DENY".to_string(),
                domains: vec!["youtube.com".into(), "netflix.com".into(), "twitch.tv".into()],
            },
            AclCategory {
                id: "corporate".to_string(),
                name: "Aplicações Corporativas".to_string(),
                description: "ERP, E-mail, Intranet, Nuvem".to_string(),
                default_action: "ALLOW".to_string(),
                domains: vec!["srvcloud.cloud".into(), "github.com".into(), "microsoft.com".into()],
            },
        ]
    }

    fn load_from_disk(path: &PathBuf) -> (Vec<AclRule>, Vec<AclCategory>) {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str::<PersistedData>(&content) {
                info!("Loaded {} ACL rules and {} categories from disk", data.rules.len(), data.categories.len());
                let cats = if data.categories.is_empty() { Self::default_categories() } else { data.categories };
                return (data.rules, cats);
            }
        }
        (vec![], Self::default_categories())
    }

    fn persist(&self) {
        let data = PersistedData {
            rules: self.rules.lock().unwrap().clone(),
            categories: self.categories.lock().unwrap().clone(),
        };

        if let Ok(serialized) = serde_json::to_string_pretty(&data) {
            if let Some(parent) = self.storage_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&self.storage_path, serialized);
        }
    }
}

impl AclEngine for StatefulAclEngine {
    fn get_rules(&self) -> Vec<AclRule> {
        self.rules.lock().unwrap().clone()
    }

    fn add_rule(&self, mut rule: AclRule) -> Result<AclRule, TrafficGuardError> {
        if rule.id.is_empty() {
            rule.id = Uuid::new_v4().to_string();
        }
        if rule.created_at.is_empty() {
            rule.created_at = Utc::now().to_rfc3339();
        }

        self.rules.lock().unwrap().push(rule.clone());
        self.persist();
        Ok(rule)
    }

    fn delete_rule(&self, id: &str) -> Result<bool, TrafficGuardError> {
        let mut rules = self.rules.lock().unwrap();
        if let Some(pos) = rules.iter().position(|r| r.id == id) {
            rules.remove(pos);
            drop(rules);
            self.persist();
            Ok(true)
        } else {
            Err(TrafficGuardError::RuleNotFound(id.to_string()))
        }
    }

    fn get_categories(&self) -> Vec<AclCategory> {
        self.categories.lock().unwrap().clone()
    }

    fn evaluate(&self, req: &CheckRequest) -> CheckResponse {
        *self.total_requests.lock().unwrap() += 1;

        let mut sorted_rules = self.rules.lock().unwrap().clone();
        sorted_rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        for rule in sorted_rules {
            if !rule.enabled {
                continue;
            }

            if rule.matches_domain(&req.domain) && rule.matches_group(req.group.as_deref()) {
                let allowed = rule.action == "ALLOW";
                if !allowed {
                    *self.blocked_requests.lock().unwrap() += 1;
                }

                return CheckResponse {
                    allowed,
                    action: rule.action.clone(),
                    matched_rule: Some(rule.name.clone()),
                    reason: format!("Matched ACL rule: {} ({})", rule.name, rule.action),
                };
            }
        }

        // Category fallback
        for cat in self.categories.lock().unwrap().iter() {
            if cat.domains.iter().any(|d| req.domain.contains(d)) {
                let allowed = cat.default_action == "ALLOW";
                if !allowed {
                    *self.blocked_requests.lock().unwrap() += 1;
                }

                return CheckResponse {
                    allowed,
                    action: cat.default_action.clone(),
                    matched_rule: Some(cat.name.clone()),
                    reason: format!("Category default: {} ({})", cat.name, cat.default_action),
                };
            }
        }

        // Default allow
        CheckResponse {
            allowed: true,
            action: "ALLOW".to_string(),
            matched_rule: None,
            reason: "Default allow policy".to_string(),
        }
    }

    fn get_status(&self) -> ProxyStatus {
        ProxyStatus {
            active: true,
            uptime_seconds: self.start_time.elapsed().as_secs(),
            total_requests: *self.total_requests.lock().unwrap(),
            blocked_requests: *self.blocked_requests.lock().unwrap(),
            rules_count: self.rules.lock().unwrap().len(),
            categories_count: self.categories.lock().unwrap().len(),
        }
    }
}
