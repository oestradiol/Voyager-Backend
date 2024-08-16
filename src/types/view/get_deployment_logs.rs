use serde::{Deserialize, Serialize};
use super::logs::Logs;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeploymentLogs {
  pub logs: Logs,
  #[serde(rename = "deploymentLogs")]
  pub deployment_logs: Option<Vec<String>>,
}