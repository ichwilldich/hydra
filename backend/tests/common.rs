use std::{env::set_var, net::TcpListener, time::Duration};

use backend::App;
use reqwest::Client;
use tokio::{spawn, time::sleep};
use uuid::Uuid;

fn find_port(used: Option<u16>) -> u16 {
  (8000..16000)
    .find(|port| used != Some(*port) && TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok())
    .expect("No ports free")
}

pub fn prepare_env() -> Port {
  let port = find_port(None);
  let storage_path = format!("/tmp/hydra-test-{}", Uuid::new_v4());

  unsafe {
    set_var("PORT", port.to_string());
    set_var("STORAGE_PATH", &storage_path);
  }

  Port { port }
}

pub struct Port {
  pub port: u16,
}

pub async fn launch_app() {
  let app = App::new().await;
  spawn(app.run());
  sleep(Duration::from_millis(100)).await; // wait for server to start
}

pub async fn run() -> Port {
  let ports = prepare_env();
  launch_app().await;
  ports
}

pub fn reqwest_client() -> Client {
  Client::builder()
    .timeout(Duration::from_secs(10))
    .connect_timeout(Duration::from_secs(10))
    .build()
    .unwrap()
}
