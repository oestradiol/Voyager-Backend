use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeploymentMode {
  Preview,
  Production,
}
impl fmt::Display for DeploymentMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Preview => write!(f, "Preview"),
      Self::Production => write!(f, "Production"),
    }
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
  #[serde(rename = "_id")]
  pub id: String,
  pub container_id: String,
  pub internal_port: u16,
  pub mode: DeploymentMode,
  pub host: String,
  pub repo_url: String,
  pub branch: String,
  pub dns_record_id: String,
}
