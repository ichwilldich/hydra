use axum::Router;
use centaurus::{
  init::{
    axum::{add_base_layers, listener_setup, run_app},
    logging::init_logging,
    metrics::{init_metrics, metrics_route},
  },
  req::health,
};
use kube::Client;
use tokio::{join, net::TcpListener};
use tracing::{info, instrument};

use crate::config::Config;

pub use crd::crds;

mod config;
mod controller;
mod crd;

pub struct App {
  app: Router,
  listener: TcpListener,
  pub kube: Option<Client>,
}

impl App {
  #[instrument]
  pub async fn new() -> App {
    let config = Config::parse();
    init_logging(&config.base);
    let handle = init_metrics(config.metrics_name.clone());

    let metrics_enabled = config.metrics_enabled;
    let metrics_name = config.metrics_name.clone();
    let metrics_labels = config.metrics_labels.clone();

    let listener = listener_setup(config.base.port).await;

    let mut app = router().await.add_base_layers(&config.base).await;

    if metrics_enabled {
      use centaurus::init::metrics::metrics;
      app = app.metrics(metrics_name, handle, metrics_labels).await;
    }

    Self {
      app,
      listener,
      kube: None,
    }
  }

  #[instrument(skip(self))]
  pub async fn run(self) {
    info!("Starting sever");
    join!(run_app(self.listener, self.app), controller::run(self.kube));
  }

  pub fn port(&self) -> u16 {
    self.listener.local_addr().unwrap().port()
  }
}

#[instrument]
async fn router() -> Router {
  Router::new().merge(health::router()).metrics_route().await
}
