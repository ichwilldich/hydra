use std::{collections::HashMap, time::Instant};

use bollard::{
  Docker,
  query_parameters::{
    CreateContainerOptions, ListContainersOptions, ListContainersOptionsBuilder,
    ListVolumesOptions, RemoveContainerOptions, RemoveVolumeOptions, StartContainerOptions,
  },
  secret::{
    ContainerCreateBody, ContainerCreateResponse, ContainerSummary, HostConfig, Volume,
    VolumeCreateOptions,
  },
};
use centaurus::error::Result;
use eyre::{Context, eyre};
use tracing::debug;
use uuid::Uuid;

use crate::connector::{
  Connector, Deployment, DeploymentInfo, DeploymentType, docker::transfer::DockerTransferExt,
};

mod transfer;

const DOCKER_MANAGED_LABEL_KEY: &str = "hydra.managed";
const DOCKER_MANAGED_LABEL_VALUE: &str = "true";
const DOCKER_DEPLOYMENT_ID_LABEL_KEY: &str = "hydra.deployment_id";
const DOCKER_DEPLOYMENT_NAME_LABEL_KEY: &str = "hydra.deployment_name";

pub struct DockerConnector {
  docker: Docker,
  filters: HashMap<String, Vec<String>>,
  default_labels: HashMap<String, String>,
}

#[async_trait::async_trait]
impl Connector for DockerConnector {
  async fn create_deployment(&self, deployment: Deployment) -> Result<()> {
    let name = format!("{}-{}", deployment.name, deployment.uuid);
    debug!("Creating deployment: {}", name);

    let mut labels = self.default_labels.clone();
    labels.insert(
      DOCKER_DEPLOYMENT_ID_LABEL_KEY.to_string(),
      deployment.uuid.to_string(),
    );
    labels.insert(
      DOCKER_DEPLOYMENT_NAME_LABEL_KEY.to_string(),
      deployment.name.clone(),
    );

    debug!("Creating volume: {}", name);
    let volume = self.create_volume(name.clone(), labels.clone()).await?;

    let data_path;
    let image;
    let env_vars;

    match deployment.typ {
      DeploymentType::Postgres => {
        data_path = "/var/lib/postgresql";
        image = "postgres:latest";
        env_vars = vec![
          "POSTGRES_USER=hydra".to_string(),
          "POSTGRES_PASSWORD=hydra".to_string(),
          "POSTGRES_DB=hydra".to_string(),
        ];
      }
    }

    let config = ContainerCreateBody {
      image: Some(image.to_string()),
      env: Some(env_vars),
      labels: Some(labels),
      host_config: Some(HostConfig {
        binds: Some(vec![format!("{}:{}", volume.name, data_path)]),
        ..Default::default()
      }),
      ..Default::default()
    };

    debug!("Creating container: {}", name);
    let container = self.create_container(&name, config).await?;

    debug!("Starting container: {}", name);
    self.start_container(&container.id).await?;

    Ok(())
  }

  async fn delete_deployment(&self, uuid: Uuid) -> Result<()> {
    debug!("Deleting deployment: {}", uuid);

    let containers = self.list_deployment_containers(uuid).await?;
    for container in containers {
      if let Some(container_id) = container.id {
        debug!("Removing container: {}", container_id);
        self.remove_container(&container_id).await?;
      }
    }

    let volumes = self.list_deployment_volumes(uuid).await?;
    for volume in volumes {
      debug!("Removing volume: {}", volume.name);
      self.remove_volume(&volume.name).await?;
    }
    Ok(())
  }

  async fn list_deployments(&self) -> Result<Vec<DeploymentInfo>> {
    let containers = self.list_deployments(self.filters.clone()).await?;
    let mut deployments: Vec<DeploymentInfo> = Vec::new();

    for container in containers {
      if let Some(labels) = container.labels
        && let Some(id_str) = labels.get(DOCKER_DEPLOYMENT_ID_LABEL_KEY)
        && let Some(name) = labels.get(DOCKER_DEPLOYMENT_NAME_LABEL_KEY)
        && let Ok(uuid) = Uuid::parse_str(id_str)
        && !deployments.iter().any(|d| d.uuid == uuid)
      {
        let deployment = DeploymentInfo {
          uuid,
          name: name.clone(),
          typ: DeploymentType::Postgres,
        };

        deployments.push(deployment);
      }
    }

    Ok(deployments)
  }
}

impl DockerConnector {
  pub fn new() -> Result<Self> {
    let docker = Docker::connect_with_defaults().context("Failed to connect to docker socket")?;

    let filters = HashMap::from([(
      "label".to_string(),
      vec![format!(
        "{}={}",
        DOCKER_MANAGED_LABEL_KEY, DOCKER_MANAGED_LABEL_VALUE
      )],
    )]);

    let default_labels = HashMap::from([(
      DOCKER_MANAGED_LABEL_KEY.to_string(),
      DOCKER_MANAGED_LABEL_VALUE.to_string(),
    )]);

    Ok(Self {
      docker,
      filters,
      default_labels,
    })
  }

  fn deployment_filter(&self, uuid: Uuid) -> HashMap<String, Vec<String>> {
    let mut filters = self.filters.clone();
    filters
      .entry("label".to_string())
      .or_default()
      .push(format!("{}={}", DOCKER_DEPLOYMENT_ID_LABEL_KEY, uuid));
    filters
  }

  async fn create_volume(&self, name: String, labels: HashMap<String, String>) -> Result<Volume> {
    let config = VolumeCreateOptions {
      name: Some(name),
      labels: Some(labels),
      ..Default::default()
    };

    let volume = self
      .docker
      .create_volume(config)
      .await
      .context("Failed to create volume")?;

    Ok(volume)
  }

  async fn create_container(
    &self,
    name: &str,
    config: ContainerCreateBody,
  ) -> Result<ContainerCreateResponse> {
    let options = Some(CreateContainerOptions {
      name: Some(name.to_string()),
      ..Default::default()
    });

    let container = self
      .docker
      .create_container(options, config)
      .await
      .with_context(|| format!("Failed to create container {}", name))?;

    Ok(container)
  }

  async fn start_container(&self, id: &str) -> Result<()> {
    self
      .docker
      .start_container(id, None::<StartContainerOptions>)
      .await
      .with_context(|| format!("Failed to start container {}", id))?;

    Ok(())
  }

  async fn list_deployment_containers(&self, uuid: Uuid) -> Result<Vec<ContainerSummary>> {
    let filters = self.deployment_filter(uuid);
    self.list_deployments(filters).await
  }

  async fn list_deployments(
    &self,
    filters: HashMap<String, Vec<String>>,
  ) -> Result<Vec<ContainerSummary>> {
    let options = ListContainersOptions {
      all: true,
      filters: Some(filters),
      ..Default::default()
    };

    let containers = self
      .docker
      .list_containers(Some(options))
      .await
      .context("Failed to list deployment containers")?;

    Ok(containers)
  }

  async fn list_deployment_volumes(&self, uuid: Uuid) -> Result<Vec<Volume>> {
    let filters = self.deployment_filter(uuid);

    let options = ListVolumesOptions {
      filters: Some(filters),
    };

    let volumes = self
      .docker
      .list_volumes(Some(options))
      .await
      .context("Failed to list deployment volumes")?;

    let volumes = volumes
      .volumes
      .ok_or(eyre!("Failed to parse volume list"))?;

    Ok(volumes)
  }

  async fn remove_container(&self, id: &str) -> Result<()> {
    let options = RemoveContainerOptions {
      force: true,
      ..Default::default()
    };

    self
      .docker
      .remove_container(id, Some(options))
      .await
      .with_context(|| format!("Failed to remove container {}", id))?;

    Ok(())
  }

  async fn remove_volume(&self, name: &str) -> Result<()> {
    self
      .docker
      .remove_volume(name, None::<RemoveVolumeOptions>)
      .await
      .with_context(|| format!("Failed to remove volume {}", name))?;

    Ok(())
  }

  pub async fn copy(&self) -> Result<()> {
    let start = Instant::now();
    let data = b"Hello, Docker!";

    let containers = self.list().await?;
    for container in containers {
      if let Some(container_id) = container.id {
        self
          .docker
          .upload_file(&container_id, "/data", "test.txt", data.to_vec())
          .await?;
      }
    }
    tracing::info!("Uploaded tar in {:?}", start.elapsed());

    Ok(())
  }

  pub async fn download(&self) -> Result<()> {
    let start = Instant::now();
    let containers = self.list().await?;

    for container in containers {
      if let Some(container_id) = container.id {
        let data = self
          .docker
          .download_file(&container_id, "/data/test.txt")
          .await?;
        tracing::info!(
          "Downloaded data from container {}: {:?}",
          container_id,
          String::from_utf8_lossy(&data)
        );
      }
    }
    tracing::info!("Downloaded file in {:?}", start.elapsed());

    Ok(())
  }

  pub async fn list(&self) -> Result<Vec<ContainerSummary>> {
    let start = Instant::now();
    let containers = self
      .docker
      .list_containers(Some(
        ListContainersOptionsBuilder::new()
          .filters(&self.filters)
          .build(),
      ))
      .await
      .context("Failed to list containers")?;
    tracing::info!("Listed containers in {:?}", start.elapsed());

    if containers.is_empty() {
      tracing::info!("No managed containers found.");
    }
    for container in &containers {
      tracing::info!("Container ID: {:?}", container.id);
      tracing::info!("Image: {:?}", container.image);
    }

    Ok(containers)
  }
}
