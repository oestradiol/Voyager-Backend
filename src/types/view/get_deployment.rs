use crate::types::model::deployment::Deployment;
use serde::{Deserialize, Serialize};
use serde_json;
use tracing::{event, Level};

use super::logs::Logs;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeployment {
  pub logs: Logs,
  pub deployment: Option<Deployment>,
}
