use std::{env::set_var, time::Duration};

use controller::{App, apply_crds};
use envtest::Environment;
use kube::{Client, config::Kubeconfig};
use tokio::{spawn, time::sleep};

pub fn prepare_env() {
  unsafe {
    set_var("PORT", "0");
  }
}

pub struct Port {
  pub port: u16,
}

pub async fn init_control_plane() -> Client {
  let env = Environment::default();
  let server = env.create().expect("Failed to create test environment");
  let kubeconfig: Kubeconfig = server.kubeconfig().expect("Failed to get kubeconfig");
  let client = Client::try_from(kubeconfig).expect("Failed to create kube client");
  apply_crds(client.clone())
    .await
    .expect("Failed to apply CRDs");
  client
}

pub async fn launch_app() -> Port {
  let mut app = App::new().await;
  let port = app.port();

  let client = init_control_plane().await;
  app.kube = client;

  spawn(app.run());
  sleep(Duration::from_millis(100)).await; // wait for server to start

  Port { port }
}

pub async fn run() -> Port {
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
