use axum::{Json, Router, routing::get};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use tracing::instrument;

use crate::auth::jwt_auth::{COOKIE_NAME, JwtAuth};

pub fn router() -> Router {
  Router::new().route("/test_token", get(test_token))
}

#[instrument(skip(cookies))]
async fn test_token(auth: Option<JwtAuth>, mut cookies: CookieJar) -> (CookieJar, Json<bool>) {
  if auth.is_none() {
    let cookie = Cookie::build((COOKIE_NAME, "")).path("/").build();
    cookies = cookies.remove(cookie);

    (cookies, Json(false))
  } else {
    (cookies, Json(true))
  }
}
