use std::{env::set_var, time::Duration};

use backend::App;
use reqwest::Client;
use tokio::{spawn, time::sleep};
use uuid::Uuid;

pub fn prepare_env() {
  let storage_path = format!("/tmp/hydra-test-{}", Uuid::new_v4());

  unsafe {
    set_var("PORT", "0");
    set_var("STORAGE_PATH", &storage_path);
  }
}

pub struct Port {
  pub port: u16,
}

pub async fn launch_app() -> Port {
  let app = App::new().await;
  let port = app.port();

  spawn(app.run());
  sleep(Duration::from_millis(100)).await; // wait for server to start

  Port { port }
}

pub async fn run() -> Port {
  prepare_env();
  launch_app().await
}

pub fn reqwest_client() -> Client {
  Client::builder()
    .timeout(Duration::from_secs(10))
    .connect_timeout(Duration::from_secs(10))
    .build()
    .unwrap()
}
