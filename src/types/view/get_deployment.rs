use crate::types::model::deployment::Deployment;
use serde::{Deserialize, Serialize};

use super::logs::Logs;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeployment {
  pub logs: Logs,
  pub deployment: Option<Deployment>,
}
