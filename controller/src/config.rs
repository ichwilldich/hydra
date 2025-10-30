use centaurus::config::BaseConfig;
use figment::{
  Figment,
  providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
  pub base: BaseConfig,

  // metrics
  pub metrics_enabled: bool,
  pub metrics_name: String,
  pub metrics_labels: Vec<(String, String)>,
}

impl Config {
  #[instrument]
  pub fn parse() -> Self {
    let config = Figment::new()
      .merge(Serialized::defaults(Self::default()))
      .merge(Env::raw().global());

    config.extract().expect("failed to load configuration")
  }
}
