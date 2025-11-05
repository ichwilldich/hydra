use axum::{Json, Router, routing::get};
use serde::Serialize;
use tracing::instrument;

use crate::auth::jwt_auth::JwtAuth;

pub fn router() -> Router {
  Router::new().route("/info", get(user_info))
}

#[derive(Serialize)]
struct UserInfo {
  name: String,
  email: String,
  id: String,
}

#[instrument]
async fn user_info(auth: JwtAuth) -> Json<UserInfo> {
  Json(UserInfo {
    name: auth.name,
    email: auth.email,
    id: auth.user_id,
  })
}
