use std::collections::HashMap;

use bollard::{
  Docker,
  query_parameters::{
    CreateContainerOptions, ListContainersOptionsBuilder, RemoveContainerOptions,
    StartContainerOptions,
  },
  secret::ContainerCreateBody,
};
use centaurus::error::Result;
use eyre::Context;

const DOCKER_MANAGED_LABEL_KEY: &str = "hydra.managed";
const DOCKER_MANAGED_LABEL_VALUE: &str = "true";

pub struct DockerConnector {
  docker: Docker,
  filters: HashMap<String, Vec<String>>,
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

    Ok(Self { docker, filters })
  }

  pub async fn list(&self) -> Result<()> {
    let containers = self
      .docker
      .list_containers(Some(
        ListContainersOptionsBuilder::new()
          .filters(&self.filters)
          .build(),
      ))
      .await
      .context("Failed to list containers")?;

    if containers.is_empty() {
      tracing::info!("No managed containers found.");
    }
    for container in containers {
      tracing::info!("Container ID: {}", container.id.unwrap_or_default());
      tracing::info!("Image: {}", container.image.unwrap_or_default());
    }

    let config = ContainerCreateBody {
      image: Some("alpine:latest".to_string()),
      cmd: Some(vec!["echo".to_string(), "hello world".to_string()]),
      labels: Some(HashMap::from([(
        DOCKER_MANAGED_LABEL_KEY.to_string(),
        DOCKER_MANAGED_LABEL_VALUE.to_string(),
      )])),
      ..Default::default()
    };

    let container = self
      .docker
      .create_container(None::<CreateContainerOptions>, config)
      .await
      .context("Failed to create container")?;

    self
      .docker
      .start_container(&container.id, None::<StartContainerOptions>)
      .await
      .context("Failed to start container")?;

    let containers = self
      .docker
      .list_containers(Some(
        ListContainersOptionsBuilder::new()
          .filters(&self.filters)
          .build(),
      ))
      .await
      .context("Failed to list containers")?;

    if containers.is_empty() {
      tracing::info!("No managed containers found.");
    }
    for container in containers {
      tracing::info!("Container ID: {}", container.id.unwrap_or_default());
      tracing::info!("Image: {}", container.image.unwrap_or_default());
    }

    self
      .docker
      .remove_container(&container.id, None::<RemoveContainerOptions>)
      .await
      .context("Failed to remove container")?;

    let containers = self
      .docker
      .list_containers(Some(
        ListContainersOptionsBuilder::new()
          .filters(&self.filters)
          .build(),
      ))
      .await
      .context("Failed to list containers")?;

    if containers.is_empty() {
      tracing::info!("No managed containers found.");
    } else {
      for container in containers {
        tracing::info!("Container ID: {}", container.id.unwrap_or_default());
        tracing::info!("Image: {}", container.image.unwrap_or_default());
      }
    }

    Ok(())
  }
}
