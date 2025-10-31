use axum::Router;

mod info;

pub fn router() -> Router {
  Router::new().merge(info::router())
}
