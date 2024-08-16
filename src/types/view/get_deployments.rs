use serde::{Deserialize, Serialize};
use super::logs::Logs;
use crate::types::model::deployment::Deployment;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeployments {
  pub logs: Logs,
  pub deployments: Vec<Deployment>
}
