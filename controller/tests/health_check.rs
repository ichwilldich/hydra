use crate::common::run;

#[allow(unused)]
mod common;

#[tokio::test]
async fn test_health_check() {
  let ports = run().await;

  let response = common::reqwest_client()
    .get(format!("http://localhost:{}/health", ports.port))
    .send()
    .await
    .unwrap();
  assert!(response.status().is_success());
}
