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

    let server = TrafficGuardServer::new(config, acl_controller, proxy_controller);
    server.run().await
}
