use serde::{Deserialize, Serialize};
use super::logs::Logs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeployment {
  pub logs: Logs,
  pub id: Option<String>,
}