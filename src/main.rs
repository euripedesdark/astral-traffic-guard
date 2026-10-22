mod config;
mod controller;
mod domain;
mod server;
mod service;

use config::TrafficGuardConfig;
use controller::{AclController, ProxyController};
use server::TrafficGuardServer;
use service::{LdapProxyAuthenticator, StatefulAclEngine};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Initializing Astral Traffic Guard (Rust OOP Architecture)...");

    let config = Arc::new(TrafficGuardConfig::load());
    let acl_engine = Arc::new(StatefulAclEngine::new(config.storage_path.clone()));
    let authenticator = Arc::new(LdapProxyAuthenticator::new(
        config.ldap_url.clone(),
        config.ldap_domain.clone(),
    ));

    let acl_controller = Arc::new(AclController::new(acl_engine));
    let proxy_controller = Arc::new(ProxyController::new(authenticator));

    let secure_proxy: Option<Arc<dyn service::proxy_engine::ProxyService>> =
        match (std::env::var("ASTRAL_DATABASE_URL"),std::env::var("AUTH_SERVICE_INTROSPECT_URL"),std::env::var("AUTH_SERVICE_CLIENT_TOKEN")) {
            (Ok(url),Ok(auth_url),Ok(secret)) => {
                let pool = sqlx::PgPool::connect(&url).await?;
                let svc = service::proxy_engine::PostgresProxyService::new(pool,auth_url,secret)?;
                Some(Arc::new(svc))
            },
            _ => {
                tracing::warn!("Secure proxy disabled: database/auth-service integration is not configured");
                None
            }
        };
    let policy_evaluator: Option<Arc<dyn service::policy_engine::PolicyEvaluator>> =
        if secure_proxy.is_some() {
            let url=std::env::var("ASTRAL_DATABASE_URL")?;
            let pool=sqlx::PgPool::connect(&url).await?;
            Some(Arc::new(service::policy_engine::PostgresPolicyEvaluator::new(pool)))
        } else { None };
    let server = TrafficGuardServer::new(config, acl_controller, proxy_controller, secure_proxy, policy_evaluator);
    server.run().await
}
