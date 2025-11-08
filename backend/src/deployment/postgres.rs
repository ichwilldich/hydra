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
}

async fn system_info() -> Result<Json<SystemInfo>> {
  let info = SystemInfo {
    connector: ConnectorType::Docker,
  };

  Ok(Json(info))
}

#[derive(FromRequest, Deserialize)]
#[from_request(via(Json))]
struct CreateDeployment {
  name: String,
  storage_mb: u64,
}

async fn create_deployment(conn: PlatformConnection, payload: CreateDeployment) -> Result<()> {
  let deployment = Deployment {
    name: payload.name,
    uuid: Uuid::new_v4(),
    typ: DeploymentType::Postgres,
    storage: StorageOptions {
      size_mb: payload.storage_mb,
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
