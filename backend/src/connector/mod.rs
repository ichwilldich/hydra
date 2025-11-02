use std::ops::Deref;

use centaurus::error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::connector::docker::DockerConnector;

pub mod docker;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorType {
  Docker,
}

pub struct PlatformConnection(Box<dyn Connector>);

impl Deref for PlatformConnection {
  type Target = dyn Connector;

  fn deref(&self) -> &Self::Target {
    &*self.0
  }
}

impl ConnectorType {
  pub fn init(&self) -> Result<PlatformConnection> {
    let connector = match self {
      ConnectorType::Docker => Box::new(DockerConnector::new()?),
    };

    Ok(PlatformConnection(connector))
  }
}

#[async_trait::async_trait]
pub trait Connector {
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

#[derive(Clone, Debug)]
pub enum DeploymentType {
  Postgres,
}

#[derive(Clone, Debug)]
pub struct StorageOptions {
  pub size_mb: u64,
}

#[derive(Clone, Debug)]
pub struct DeploymentInfo {
  pub uuid: Uuid,
  pub name: String,
  pub typ: DeploymentType,
}
