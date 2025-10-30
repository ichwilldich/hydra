use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{CustomResource, CustomResourceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
  kind = "Document",
  group = "kube.rs",
  version = "v1alpha1",
  status = "DocumentStatus",
  shortname = "doc",
  namespaced
)]
pub struct DocumentSpec {
  pub foo: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DocumentStatus {
  pub is_ready: bool,
}

pub fn crds() -> Vec<CustomResourceDefinition> {
  vec![Document::crd()]
}
