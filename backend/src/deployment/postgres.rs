use std::collections::HashMap;

use axum::{
  Json, Router,
  extract::FromRequest,
  routing::{delete, get, post},
};
use centaurus::error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::connector::{
  ConnectorType, Deployment, DeploymentInfo, DeploymentType, PlatformConnection, StorageOptions,
};

pub fn router() -> Router {
  Router::new()
    .route("/info", get(system_info))
    .route("/", post(create_deployment))
    .route("/", get(list_deployments))
    .route("/", delete(remove_deployment))
}

#[derive(Serialize)]
struct SystemInfo {
  connector: ConnectorType,
  namespaces: Vec<String>,
}

async fn system_info() -> Result<Json<SystemInfo>> {
  let info = SystemInfo {
    connector: ConnectorType::Docker,
    namespaces: vec![],
  };

  Ok(Json(info))
}

#[derive(FromRequest, Deserialize)]
#[from_request(via(Json))]
struct CreateDeployment {
  name: String,
  namespace: Option<String>,
  version: PostgresVersion,
  replicas: u32,
  resources: DeploymentResources,
  backup: DeploymentBackup,
  connection: DeploymentConnection,
  monitoring: DeploymentMonitoring,
  advanced: DeploymentAdvanced,
}

#[derive(Deserialize)]
struct DeploymentResources {
  storage_mb: Option<u64>,
  memory_request_mb: u64,
  memory_limit_mb: u64,
  cpu_request_millicores: u64,
  cpu_limit_millicores: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "enabled")]
enum DeploymentBackup {
  Disabled,
  Enabled {
    location: String,
    schedule: String,
    retention_days: u32,
  },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "ssl_enabled")]
enum DeploymentConnection {
  Disabled {
    external_access: bool,
  },
  Enabled {
    external_access: bool,
    ssl_cert: SslFile,
    ssl_key: SslFile,
    ssl_ca: SslCa,
  },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "ca_enabled")]
enum SslCa {
  Disabled,
  Enabled { ssl_ca: SslFile },
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum SslFile {
  Auto,
  Text { content: String },
  Reference { ref_name: String, ref_key: String },
  HostFilePath { host_file_path: String },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "enabled")]
enum DeploymentMonitoring {
  Disabled,
  Enabled {
    external_access: bool,
    deploy_monitoring: bool,
  },
}

#[derive(Deserialize)]
struct DeploymentAdvanced {
  allow_alter_system: bool,
  extra_params: HashMap<String, String>,
}

#[derive(Deserialize)]
enum PostgresVersion {
  #[serde(rename = "13")]
  V13,
  #[serde(rename = "14")]
  V14,
  #[serde(rename = "15")]
  V15,
  #[serde(rename = "16")]
  V16,
  #[serde(rename = "17")]
  V17,
  #[serde(rename = "18")]
  V18,
}

async fn create_deployment(conn: PlatformConnection, payload: CreateDeployment) -> Result<()> {
  let deployment = Deployment {
    name: payload.name,
    uuid: Uuid::new_v4(),
    typ: DeploymentType::Postgres,
    storage: StorageOptions {
      size_mb: payload.resources.storage_mb.unwrap_or(1000),
    },
  };

  conn.create_deployment(deployment).await?;

  Ok(())
}

async fn list_deployments(conn: PlatformConnection) -> Result<Json<Vec<DeploymentInfo>>> {
  let deployments = conn.list_deployments().await?;
  Ok(Json(deployments))
}

#[derive(FromRequest, Deserialize)]
#[from_request(via(Json))]
struct RemoveDeployment {
  uuid: Uuid,
}

async fn remove_deployment(conn: PlatformConnection, payload: RemoveDeployment) -> Result<()> {
  conn.delete_deployment(payload.uuid).await
}
