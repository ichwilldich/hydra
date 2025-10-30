use crate::common::run;

mod common;

#[tokio::test]
async fn test_health_check() {
  let ports = run().await;

  let response = common::reqwest_client()
    .get(format!("http://localhost:{}/api/health", ports.port))
    .send()
    .await
    .unwrap();
  assert!(response.status().is_success());
}
