use serde::{Deserialize, Serialize};
use crate::types::model::deployment::Deployment;

use super::logs::Logs;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeployment {
  pub logs: Logs,
  pub deployment: Option<Deployment>,
}