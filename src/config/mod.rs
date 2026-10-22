use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct TrafficGuardConfig {
    pub api_port: u16,
    pub authproxy_port: u16,
    pub storage_path: PathBuf,
    pub ldap_url: String,
    pub ldap_domain: String,
}

impl TrafficGuardConfig {
    pub fn load() -> Self {
        let api_port: u16 = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8085);

        let authproxy_port: u16 = std::env::var("AUTH_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8091);

        let storage_path = PathBuf::from("/etc/astral/traffic-guard.json");
        let ldap_url = std::env::var("ASTRAL_AD_URL").unwrap_or_else(|_| "ldaps://127.0.0.1:636".to_string());
        let ldap_domain = std::env::var("ASTRAL_AD_DOMAIN").unwrap_or_else(|_| "srvcloud.cloud".to_string());

        Self {
            api_port,
            authproxy_port,
            storage_path,
            ldap_url,
            ldap_domain,
        }
    }
}
