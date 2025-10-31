use serde::{Deserialize, Serialize};

pub mod docker;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorType {
  Docker,
}
