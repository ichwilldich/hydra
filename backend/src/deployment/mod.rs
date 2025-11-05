use axum::Router;

mod postgres;

pub fn router() -> Router {
  Router::new().nest("/postgres", postgres::router())
}
