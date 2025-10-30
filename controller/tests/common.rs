use std::{env::set_var, time::Duration};

use controller::{App, crds};
use envtest::{Environment, Server};
use kube::{Client, config::Kubeconfig};
use tokio::{spawn, time::sleep};
use tracing::info;

pub fn prepare_env() {
  unsafe {
    set_var("PORT", "0");
  }
}

/// This needs to be in scope for the entire test so the fake control plane
/// is not dropped while the test is running.
pub struct Env {
  pub port: u16,
  // required to keep the test environment alive
  #[allow(dead_code)]
  server: Server,
  pub client: Client,
}

impl Drop for Env {
  fn drop(&mut self) {
    info!("Tearing down test control plane");
  }
}

pub fn init_control_plane() -> (Server, Client) {
  info!("Initializing test control plane");
  let mut env = Environment::default();
  env.with_crds(crds()).expect("Failed to add CRDs");

  let server = env.create().expect("Failed to create test environment");
  let kubeconfig: Kubeconfig = server.kubeconfig().expect("Failed to get kubeconfig");
  let client = Client::try_from(kubeconfig).expect("Failed to create kube client");

  info!("Test control plane initialized");
  (server, client)
}

pub async fn launch_app() -> Env {
  let mut app = App::new().await;
  let port = app.port();

  let (server, client) = init_control_plane();
  app.kube = client.clone();

  spawn(app.run());
  sleep(Duration::from_millis(100)).await; // wait for server to start

  Env {
    port,
    server,
    client,
  }
}

pub async fn run() -> Env {
  prepare_env();
  launch_app().await
}

pub fn reqwest_client() -> reqwest::Client {
  reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .connect_timeout(Duration::from_secs(10))
    .build()
    .unwrap()
}
