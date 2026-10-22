use crate::domain::policy::NewProxyPolicy;
use crate::service::policy_engine::PolicyEvaluator;
use crate::service::proxy_engine::ProxyService;
use axum::{extract::{Path,State},http::{header,HeaderMap,StatusCode},response::{IntoResponse,Response},routing::get,Json,Router};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PolicyState {
    pub identity:Arc<dyn ProxyService>,
    pub evaluator:Arc<dyn PolicyEvaluator>,
}
async fn admin(s:&PolicyState,h:&HeaderMap)->Result<String,Response>{
    let token=h.get(header::AUTHORIZATION).and_then(|v|v.to_str().ok()).and_then(|v|v.strip_prefix("Bearer "))
        .ok_or_else(||StatusCode::UNAUTHORIZED.into_response())?;
    let ctx=s.identity.identity(token).await.map_err(|_|StatusCode::UNAUTHORIZED.into_response())?;
    if !ctx.roles.iter().any(|r|r=="proxy_admin") {return Err(StatusCode::FORBIDDEN.into_response());}
    Ok(ctx.identity_id)
}
fn fail(e:crate::service::proxy_engine::ProxyError)->Response{
    tracing::warn!(error=%e,"policy operation failed");
    (StatusCode::SERVICE_UNAVAILABLE,Json(serde_json::json!({"error":"policy service unavailable or invalid policy"}))).into_response()
}
async fn list(State(s):State<PolicyState>,h:HeaderMap)->Response{
    if let Err(r)=admin(&s,&h).await {return r;}
    match s.evaluator.list().await {Ok(p)=>Json(p).into_response(),Err(e)=>fail(e)}
}
async fn create(State(s):State<PolicyState>,h:HeaderMap,Json(p):Json<NewProxyPolicy>)->Response{
    let actor=match admin(&s,&h).await{Ok(a)=>a,Err(r)=>return r};
    match s.evaluator.create(&actor,p).await {Ok(v)=>(StatusCode::CREATED,Json(v)).into_response(),Err(e)=>fail(e)}
}
async fn remove(State(s):State<PolicyState>,h:HeaderMap,Path(id):Path<Uuid>)->Response{
    let actor=match admin(&s,&h).await{Ok(a)=>a,Err(r)=>return r};
    match s.evaluator.delete(&actor,id).await{Ok(())=>StatusCode::NO_CONTENT.into_response(),Err(e)=>fail(e)}
}
pub fn router(identity:Arc<dyn ProxyService>,evaluator:Arc<dyn PolicyEvaluator>)->Router{
    Router::new().route("/api/v1/secure-proxy/policies",get(list).post(create))
        .route("/api/v1/secure-proxy/policies/{id}",axum::routing::delete(remove))
        .with_state(PolicyState{identity,evaluator})
}
