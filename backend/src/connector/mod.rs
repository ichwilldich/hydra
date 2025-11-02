use std::{ops::Deref, sync::Arc};

use axum::{Extension, extract::FromRequestParts};
use centaurus::{error::Result, router_extension};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::EnvConfig, connector::docker::DockerConnector};

pub mod docker;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorType {
  Docker,
}

#[derive(FromRequestParts, Clone)]
#[from_request(via(Extension))]
pub struct PlatformConnection(Arc<dyn Connector>);

router_extension!(
  async fn connector(self, config: &EnvConfig) -> Self {
    self.layer(Extension(
      config
        .connector
        .init()
        .expect("Failed to initialize platform connector"),
    ))
  }
);

impl Deref for PlatformConnection {
  type Target = dyn Connector;

  fn deref(&self) -> &Self::Target {
    &*self.0
  }
}

impl ConnectorType {
  pub fn init(&self) -> Result<PlatformConnection> {
    let connector = match self {
      ConnectorType::Docker => Arc::new(DockerConnector::new()?),
    };

    Ok(PlatformConnection(connector))
  }
}

#[async_trait::async_trait]
pub trait Connector: Send + Sync {
  async fn create_deployment(&self, deployment: Deployment) -> Result<()>;
  async fn delete_deployment(&self, uuid: Uuid) -> Result<()>;
  async fn list_deployments(&self) -> Result<Vec<DeploymentInfo>>;
}

#[derive(Clone, Debug)]
pub struct Deployment {
  pub uuid: Uuid,
  pub name: String,
  pub typ: DeploymentType,
  pub storage: StorageOptions,
}

#[derive(Clone, Debug, Serialize)]
pub enum DeploymentType {
  Postgres,
}

#[derive(Clone, Debug)]
pub struct StorageOptions {
  pub size_mb: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct DeploymentInfo {
  pub uuid: Uuid,
  pub name: String,
  pub typ: DeploymentType,
}
