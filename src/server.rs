use crate::config::TrafficGuardConfig;
use crate::controller::{AclController, ProxyController};
use crate::domain::{AclRule, CheckRequest};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    routing::{delete, get, post},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct TrafficGuardServer {
    config: Arc<TrafficGuardConfig>,
    acl_controller: Arc<AclController>,
    proxy_controller: Arc<ProxyController>,
    secure_proxy: Option<Arc<dyn crate::service::proxy_engine::ProxyService>>,
    policy_evaluator: Option<Arc<dyn crate::service::policy_engine::PolicyEvaluator>>,
}

impl TrafficGuardServer {
    pub fn new(
        config: Arc<TrafficGuardConfig>,
        acl_controller: Arc<AclController>,
        proxy_controller: Arc<ProxyController>,
        secure_proxy: Option<Arc<dyn crate::service::proxy_engine::ProxyService>>,
        policy_evaluator: Option<Arc<dyn crate::service::policy_engine::PolicyEvaluator>>,
    ) -> Self {
        Self {
            config,
            acl_controller,
            proxy_controller,
            secure_proxy,
            policy_evaluator,
        }
    }

    pub fn build_api_router(&self) -> Router {
        let mut router = Router::new()
            .route(
                "/health",
                get(|State(ctrl): State<Arc<AclController>>| async move { ctrl.handle_health() }),
            )
            .route(
                "/actuator/health",
                get(|State(ctrl): State<Arc<AclController>>| async move { ctrl.handle_health() }),
            )
            .route(
                "/api/v1/proxy/status",
                get(|State(ctrl): State<Arc<AclController>>| async move { ctrl.handle_status() }),
            )
            .route(
                "/api/v1/acl/rules",
                get(|State(ctrl): State<Arc<AclController>>| async move { ctrl.handle_get_rules() })
                    .post(|State(ctrl): State<Arc<AclController>>, Json(rule): Json<AclRule>| async move {
                        ctrl.handle_add_rule(rule)
                    }),
            )
            .route(
                "/api/v1/acl/rules/{id}",
                delete(|State(ctrl): State<Arc<AclController>>, Path(id): Path<String>| async move {
                    ctrl.handle_delete_rule(&id)
                }),
            )
            .route(
                "/api/v1/acl/categories",
                get(|State(ctrl): State<Arc<AclController>>| async move { ctrl.handle_get_categories() }),
            )
            .route(
                "/api/v1/acl/check",
                post(|State(ctrl): State<Arc<AclController>>, Json(req): Json<CheckRequest>| async move {
                    ctrl.handle_check(req)
                }),
            )
            .with_state(self.acl_controller.clone());
        if let Some(service) = &self.secure_proxy {
            router = router.merge(crate::controller::secure_proxy_controller::router(service.clone()));
        }
        if let (Some(service),Some(evaluator))=(&self.secure_proxy,&self.policy_evaluator) {
            router=router.merge(crate::controller::policy_controller::router(service.clone(),evaluator.clone()));
        }
        router.layer(TraceLayer::new_for_http())
    }

    pub fn build_authproxy_router(&self) -> Router {
        Router::new()
            .route(
                "/auth",
                get(|State(ctrl): State<Arc<ProxyController>>, headers: HeaderMap| async move {
                    ctrl.handle_authproxy(&headers).await
                })
                .head(|State(ctrl): State<Arc<ProxyController>>, headers: HeaderMap| async move {
                    ctrl.handle_authproxy(&headers).await
                })
                .post(|State(ctrl): State<Arc<ProxyController>>, headers: HeaderMap| async move {
                    ctrl.handle_authproxy(&headers).await
                }),
            )
            .route(
                "/",
                get(|State(ctrl): State<Arc<ProxyController>>, headers: HeaderMap| async move {
                    ctrl.handle_authproxy(&headers).await
                })
                .head(|State(ctrl): State<Arc<ProxyController>>, headers: HeaderMap| async move {
                    ctrl.handle_authproxy(&headers).await
                })
                .post(|State(ctrl): State<Arc<ProxyController>>, headers: HeaderMap| async move {
                    ctrl.handle_authproxy(&headers).await
                }),
            )
            .with_state(self.proxy_controller.clone())
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api_addr = SocketAddr::from(([0, 0, 0, 0], self.config.api_port));
        let auth_addr = SocketAddr::from(([0, 0, 0, 0], self.config.authproxy_port));

        info!("TrafficGuardServer listening API on http://{}", api_addr);
        info!("TrafficGuardServer listening AuthProxy on http://{}", auth_addr);

        let api_listener = tokio::net::TcpListener::bind(api_addr).await?;
        let auth_listener = tokio::net::TcpListener::bind(auth_addr).await?;

        let api_router = self.build_api_router();
        let auth_router = self.build_authproxy_router();

        let srv1 = tokio::spawn(async move { axum::serve(api_listener, api_router).await });
        let srv2 = tokio::spawn(async move { axum::serve(auth_listener, auth_router).await });

        let _ = tokio::try_join!(srv1, srv2)?;
        Ok(())
    }
}
