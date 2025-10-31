use centaurus::init::logging::init_logging;
use clap::Parser;

use crate::{App, config::EnvConfig, connector::docker::DockerConnector};

/// Hydra CLI
#[derive(Parser)]
#[command(version, about)]
pub enum Cli {
  /// Start Hydra server. Mainly used as the docker container entrypoint.
  Server,
  Test,
}

impl Cli {
  pub async fn run(self) {
    if !matches!(self, Cli::Server) {
      let config = EnvConfig::parse();
      init_logging(&config.base);
    }

    match self {
      Cli::Server => {
        let app = App::new().await;
        app.run().await;
      }
      Cli::Test => {
        let conn = DockerConnector::new().expect("failed to create docker connector");
        conn.list().await.expect("failed to list docker containers");
      }
    }
  }
}
