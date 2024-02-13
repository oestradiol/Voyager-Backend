use serde::Serialize;
use crate::utils::http_client::ensure_success::EnsureSuccess;

#[derive(Debug, Serialize)]
pub enum DeploymentMode {
  Preview,
  Production,
}
