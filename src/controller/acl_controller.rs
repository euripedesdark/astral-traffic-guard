use crate::domain::{AclRule, CheckRequest};
use crate::service::AclEngine;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct AclController {
    engine: Arc<dyn AclEngine>,
}

impl AclController {
    pub fn new(engine: Arc<dyn AclEngine>) -> Self {
        Self { engine }
    }

    pub fn handle_health(&self) -> Json<serde_json::Value> {
        Json(json!({ "status": "UP", "service": "astral-traffic-guard" }))
    }

    pub fn handle_status(&self) -> Json<serde_json::Value> {
        Json(json!(self.engine.get_status()))
    }

    pub fn handle_get_rules(&self) -> Json<serde_json::Value> {
        Json(json!(self.engine.get_rules()))
    }

    pub fn handle_add_rule(&self, rule: AclRule) -> Response {
        match self.engine.add_rule(rule) {
            Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
            Err(e) => e.to_json_response().into_response(),
        }
    }

    pub fn handle_delete_rule(&self, id: &str) -> Response {
        match self.engine.delete_rule(id) {
            Ok(_) => (StatusCode::OK, Json(json!({"deleted": true, "id": id}))).into_response(),
            Err(e) => e.to_json_response().into_response(),
        }
    }

    pub fn handle_get_categories(&self) -> Json<serde_json::Value> {
        Json(json!(self.engine.get_categories()))
    }

    pub fn handle_check(&self, req: CheckRequest) -> Json<serde_json::Value> {
        let resp = self.engine.evaluate(&req);
        Json(json!(resp))
    }
}
