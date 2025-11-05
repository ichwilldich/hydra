use axum::{Extension, Router};
use centaurus::{
  db::init::init_db,
  init::{
    axum::{add_base_layers, listener_setup, run_app},
    logging::init_logging,
    metrics::init_metrics,
  },
  req::health,
  router_extension,
};
use tokio::{fs, net::TcpListener};
use tracing::{info, instrument};

pub use crate::cli::Cli;
use crate::config::{AppConfig, EnvConfig};

mod auth;
mod cli;
mod config;
mod connector;
mod db;
mod deployment;
mod frontend;
mod user;

#[derive(Debug)]
pub struct App {
  app: Router,
  app_listener: TcpListener,
}

impl App {
  #[instrument]
  pub async fn new() -> App {
    let config = EnvConfig::parse();
    init_logging(&config.base);
    let handle = init_metrics(config.metrics_name.clone());

    fs::create_dir_all(&config.storage_path)
      .await
      .expect("failed to create storage path");

    let metrics_enabled = config.metrics_enabled;
    let metrics_name = config.metrics_name.clone();
    let metrics_labels = config.metrics_labels.clone();

    let app_listener = listener_setup(config.base.port).await;

    let mut app = router(&config).await.state(config).await;

    if metrics_enabled {
      use centaurus::init::metrics::metrics;
      app = app.metrics(metrics_name, handle, metrics_labels).await;
    }

    Self { app, app_listener }
  }

  #[instrument(skip(self))]
  pub async fn run(self) {
    info!("Starting sever");
    run_app(self.app_listener, self.app).await;
  }

  pub fn port(&self) -> u16 {
    self.app_listener.local_addr().unwrap().port()
  }
}

#[instrument(skip(config))]
async fn router(config: &EnvConfig) -> Router {
  use centaurus::init::metrics::metrics_route;
  frontend::router()
    .nest(
      "/api",
      Router::new()
        .nest("/auth", auth::router())
        .nest("/user", user::router())
        .nest("/deployment", deployment::router())
        .merge(health::router())
        .metrics_route()
        .await,
    )
    .add_base_layers_filtered(&config.base, |path| path.starts_with("/api"))
    .await
}

router_extension!(
  async fn state(self, env_config: EnvConfig) -> Self {
    use auth::auth;
    use config::config;
    use connector::connector;
    use frontend::frontend;

    let db = init_db::<migration::Migrator>(
      &env_config.db,
      &format!(
        "sqlite:{}/sqlite.db?mode=rwc",
        env_config.storage_path.display()
      ),
    )
    .await;
    let app_config = AppConfig::new(&db).await;

    self
      .auth(&env_config, &app_config, &db)
      .await
      .frontend()
      .await
      .config(&db)
      .await
      .connector(&env_config)
      .await
      .layer(Extension(db))
      .layer(Extension(env_config))
      .layer(Extension(app_config))
  }
);

#[cfg(test)]
mod test {
  #[tokio::test]
  async fn test_router() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/hydra-test");
    }
    // test if there are any handler setup error that are not caught at compile time
    let _ = super::router(&super::EnvConfig::parse()).await;
  }
}
