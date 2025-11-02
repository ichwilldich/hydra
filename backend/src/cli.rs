use centaurus::init::logging::init_logging;
use clap::Parser;
use uuid::Uuid;

use crate::{
  App,
  config::EnvConfig,
  connector::{Connector, Deployment, StorageOptions, docker::DockerConnector},
};

/// Hydra CLI
#[derive(Parser)]
#[command(version, about)]
pub enum Cli {
  /// Start Hydra server. Mainly used as the docker container entrypoint.
  Server,
  List,
  Copy,
  Download,
  Create,
  Remove {
    uuid: Uuid,
  },
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
      Cli::List => {
        let conn = DockerConnector::new().expect("failed to create docker connector");
        dbg!(
          conn
            .list_deployments()
            .await
            .expect("failed to list docker containers")
        );
      }
      Cli::Create => {
        let conn = DockerConnector::new().expect("failed to create docker connector");
        conn
          .create_deployment(Deployment {
            name: "hydra-test".to_string(),
            typ: crate::connector::DeploymentType::Postgres,
            storage: StorageOptions { size_mb: 1000 },
            uuid: Uuid::new_v4(),
          })
          .await
          .expect("failed to create docker deployment");
      }
      Cli::Remove { uuid } => {
        let conn = DockerConnector::new().expect("failed to create docker connector");
        conn
          .delete_deployment(uuid)
          .await
          .expect("failed to remove docker deployment");
      }
      Cli::Copy => {
        let conn = DockerConnector::new().expect("failed to create docker connector");
        conn
          .copy()
          .await
          .expect("failed to copy file to docker container");
      }
      Cli::Download => {
        let conn = DockerConnector::new().expect("failed to create docker connector");
        conn
          .download()
          .await
          .expect("failed to download file from docker container");
      }
    }
  }
}
