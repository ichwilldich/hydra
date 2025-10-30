use centaurus::{anyhow, error::Result};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{Api, Client, CustomResource, CustomResourceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
  kind = "Dummy",
  group = "example.com",
  version = "v1alpha1",
  status = "DummyStatus",
  shortname = "dum",
  namespaced
)]
pub struct DummySpec {
  pub foo: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DummyStatus {
  pub is_ready: bool,
}

pub async fn apply_crds(client: Client) -> Result<()> {
  let crds = vec![Dummy::crd()];
  let api = Api::<CustomResourceDefinition>::all(client);

  for crd in crds {
    let crd_name = crd.metadata.name.clone().unwrap_or_default();
    match api.get(&crd_name).await {
      Ok(_) => {
        // CRD already exists, update it
        info!("CRD {} already exists, updating", crd_name);
        api.replace(&crd_name, &Default::default(), &crd).await?;
      }
      Err(kube::Error::Api(ae)) if ae.code == 404 => {
        // CRD does not exist, create it
        info!("Creating CRD {}", crd_name);
        api.create(&Default::default(), &crd).await?;
      }
      Err(e) => return Err(anyhow!(e)),
    }
  }
  Ok(())
}
