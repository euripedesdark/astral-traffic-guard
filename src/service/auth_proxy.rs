use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ldap3::{LdapConnAsync, LdapConnSettings};
use tracing::warn;

#[async_trait]
pub trait ProxyAuthenticator: Send + Sync {
    async fn authenticate(&self, auth_header: Option<&str>) -> bool;
}

pub struct LdapProxyAuthenticator {
    ldap_url: String,
    ldap_domain: String,
}

impl LdapProxyAuthenticator {
    pub fn new(ldap_url: impl Into<String>, ldap_domain: impl Into<String>) -> Self {
        Self {
            ldap_url: ldap_url.into(),
            ldap_domain: ldap_domain.into(),
        }
    }
}

#[async_trait]
impl ProxyAuthenticator for LdapProxyAuthenticator {
    async fn authenticate(&self, auth_header: Option<&str>) -> bool {
        let auth = match auth_header {
            Some(a) if a.starts_with("Basic ") => a[6..].trim(),
            _ => return false,
        };

        let decoded = match BASE64.decode(auth) {
            Ok(d) => match String::from_utf8(d) {
                Ok(s) => s,
                Err(_) => return false,
            },
            Err(_) => return false,
        };

        let (user, pass) = match decoded.split_once(':') {
            Some((u, p)) => (u, p),
            None => return false,
        };

        if user.is_empty() || pass.is_empty() {
            return false;
        }

        let bind_dn = if user.contains('@') {
            user.to_string()
        } else {
            format!("{}@{}", user, self.ldap_domain)
        };

        let settings = LdapConnSettings::new().set_no_tls_verify(true);
        if let Ok((conn, mut ldap)) = LdapConnAsync::with_settings(settings, &self.ldap_url).await {
            ldap3::drive!(conn);
            return ldap.simple_bind(&bind_dn, pass).await.is_ok();
        }

        false
    }
}
