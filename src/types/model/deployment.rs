use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeploymentMode {
  PREVIEW,
  PRODUCTION,
}
impl fmt::Display for DeploymentMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DeploymentMode::PREVIEW => write!(f, "Preview"),
      DeploymentMode::PRODUCTION => write!(f, "Production"),
    }
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
  pub id: String,
  pub container_id: String,
  pub internal_port: u16,
  pub mode: DeploymentMode,
  pub host: String,
  pub branch: String,
  pub dns_record_id: String,
}