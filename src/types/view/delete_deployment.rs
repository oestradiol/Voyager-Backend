use serde::{Deserialize, Serialize};
use super::logs::Logs;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeployment {
  pub logs: Logs
}