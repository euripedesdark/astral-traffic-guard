use crate::service::{feed_manager::{FeedManagerRef,NewFeed},proxy_engine::ProxyService};
use axum::{extract::{Path,State},http::{header,HeaderMap,StatusCode},response::{IntoResponse,Response},routing::get,Json,Router};
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone)]
pub struct FeedState{pub feeds:FeedManagerRef,pub auth:Arc<dyn ProxyService>}
async fn guard(s:&FeedState,h:&HeaderMap)->Result<String,Response>{
 let token=h.get(header::AUTHORIZATION).and_then(|v|v.to_str().ok()).and_then(|v|v.strip_prefix("Bearer "))
  .ok_or_else(||StatusCode::UNAUTHORIZED.into_response())?;
 let actor=s.auth.identity(token).await.map_err(|_|StatusCode::UNAUTHORIZED.into_response())?;
 if !actor.roles.iter().any(|r|r=="proxy_admin"){return Err(StatusCode::FORBIDDEN.into_response());}
 Ok(actor.identity_id)
}
fn error(e:String)->Response{
 tracing::warn!(error=%e,"feed operation failed");
 (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"feed operation failed"}))).into_response()
}
async fn feeds(State(s):State<FeedState>,h:HeaderMap)->Response{
 if let Err(r)=guard(&s,&h).await{return r}
 match s.feeds.feeds().await{Ok(v)=>Json(v).into_response(),Err(e)=>error(e)}
}
async fn categories(State(s):State<FeedState>,h:HeaderMap)->Response{
 if let Err(r)=guard(&s,&h).await{return r}
 match s.feeds.categories().await{Ok(v)=>Json(v).into_response(),Err(e)=>error(e)}
}
async fn add(State(s):State<FeedState>,h:HeaderMap,Json(feed):Json<NewFeed>)->Response{
 if let Err(r)=guard(&s,&h).await{return r}
 match s.feeds.add(feed).await{Ok(v)=>(StatusCode::CREATED,Json(v)).into_response(),Err(e)=>error(e)}
}
async fn refresh(State(s):State<FeedState>,h:HeaderMap,Path(id):Path<Uuid>)->Response{
 if let Err(r)=guard(&s,&h).await{return r}
 match s.feeds.refresh(id).await{Ok(v)=>Json(v).into_response(),Err(e)=>error(e)}
}
pub fn router(auth: Arc<dyn ProxyService>, feed_mgr: FeedManagerRef) -> Router {
    Router::new()
        .route("/api/v1/secure-proxy/feeds", get(feeds).post(add))
        .route("/api/v1/secure-proxy/categories", get(categories))
        .route("/api/v1/secure-proxy/feeds/{id}/refresh", axum::routing::post(refresh))
        .with_state(FeedState { feeds: feed_mgr, auth })
}
