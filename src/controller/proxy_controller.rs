use crate::service::ProxyAuthenticator;
use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProxyController {
    authenticator: Arc<dyn ProxyAuthenticator>,
}

impl ProxyController {
    pub fn new(authenticator: Arc<dyn ProxyAuthenticator>) -> Self {
        Self { authenticator }
    }

    pub async fn handle_authproxy(&self, headers: &HeaderMap) -> Response {
        let auth_hdr = headers
            .get("Proxy-Authorization")
            .or_else(|| headers.get(header::AUTHORIZATION))
            .and_then(|h| h.to_str().ok());

        if self.authenticator.authenticate(auth_hdr).await {
            let mut resp_headers = HeaderMap::new();
            resp_headers.insert("X-Auth-Status", HeaderValue::from_static("OK"));
            return (StatusCode::OK, resp_headers, "OK").into_response();
        }

        let mut resp_headers = HeaderMap::new();
        resp_headers.insert(
            header::PROXY_AUTHENTICATE,
            HeaderValue::from_static("Basic realm=\"Astral Traffic Guard\""),
        );
        (StatusCode::PROXY_AUTHENTICATION_REQUIRED, resp_headers, "Proxy Authentication Required").into_response()
    }
}
