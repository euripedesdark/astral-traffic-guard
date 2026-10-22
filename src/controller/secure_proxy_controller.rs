use crate::domain::proxy::{DecisionRequest, IdentityContext, NewProxyRule};
use crate::service::proxy_engine::{ProxyError, ProxyService};
use axum::{extract::{Path,State},http::{header,HeaderMap,StatusCode},response::IntoResponse,routing::{get,post},Json,Router};
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone)]
pub struct SecureProxyState { pub service: Arc<dyn ProxyService> }
fn error(e:ProxyError)->(StatusCode,Json<serde_json::Value>){
 let code=match e.0.as_str(){ "unauthorized" | "authentication unavailable"=>StatusCode::UNAUTHORIZED,"forbidden"=>StatusCode::FORBIDDEN,"not found"=>StatusCode::NOT_FOUND,"invalid proxy rule"|"invalid target"=>StatusCode::BAD_REQUEST,_=>StatusCode::SERVICE_UNAVAILABLE};
 (code,Json(serde_json::json!({"error": if code==StatusCode::SERVICE_UNAVAILABLE {"service unavailable"} else {e.0.as_str()}})))
}
async fn actor(s:&SecureProxyState,h:&HeaderMap)->Result<IdentityContext,(StatusCode,Json<serde_json::Value>)>{
 let token=h.get(header::AUTHORIZATION).and_then(|v|v.to_str().ok()).and_then(|v|v.strip_prefix("Bearer ")).ok_or_else(||error(ProxyError("unauthorized".into())))?;
 s.service.identity(token).await.map_err(error)
}
async fn list(State(s):State<SecureProxyState>,h:HeaderMap)->impl IntoResponse{
 let a=match actor(&s,&h).await {Ok(v)=>v,Err(e)=>return e.into_response()};
 if !a.roles.iter().any(|r|r=="proxy_admin"){return error(ProxyError("forbidden".into())).into_response();}
 match s.service.rules().await {Ok(v)=>Json(v).into_response(),Err(e)=>error(e).into_response()}
}
async fn create(State(s):State<SecureProxyState>,h:HeaderMap,Json(rule):Json<NewProxyRule>)->impl IntoResponse{
 let a=match actor(&s,&h).await {Ok(v)=>v,Err(e)=>return e.into_response()};
 match s.service.create_rule(&a,rule).await {Ok(v)=>(StatusCode::CREATED,Json(v)).into_response(),Err(e)=>error(e).into_response()}
}
async fn remove(State(s):State<SecureProxyState>,h:HeaderMap,Path(id):Path<Uuid>)->impl IntoResponse{
 let a=match actor(&s,&h).await {Ok(v)=>v,Err(e)=>return e.into_response()};
 match s.service.delete_rule(&a,id).await {Ok(())=>StatusCode::NO_CONTENT.into_response(),Err(e)=>error(e).into_response()}
}
async fn logs(State(s):State<SecureProxyState>,h:HeaderMap)->impl IntoResponse{
 let a=match actor(&s,&h).await {Ok(v)=>v,Err(e)=>return e.into_response()};
 if !a.roles.iter().any(|r|r=="proxy_admin" || r=="proxy_auditor"){return error(ProxyError("forbidden".into())).into_response();}
 match s.service.audit(200).await {Ok(v)=>Json(v).into_response(),Err(e)=>error(e).into_response()}
}
async fn decision(State(s):State<SecureProxyState>,h:HeaderMap,Json(q):Json<DecisionRequest>)->impl IntoResponse{
 let a=match actor(&s,&h).await {Ok(v)=>v,Err(e)=>return e.into_response()};
 match s.service.decide(&a,q).await {Ok(v)=>Json(v).into_response(),Err(e)=>error(e).into_response()}
}
pub fn router(service:Arc<dyn ProxyService>)->Router{
 Router::new().route("/api/v1/secure-proxy/rules",get(list).post(create))
 .route("/api/v1/secure-proxy/rules/{id}",axum::routing::delete(remove))
 .route("/api/v1/secure-proxy/audit",get(logs))
 .route("/api/v1/secure-proxy/decide",post(decision))
 .with_state(SecureProxyState{service})
}
