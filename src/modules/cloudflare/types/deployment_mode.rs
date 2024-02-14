use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum DeploymentMode {
  Preview,
  Production,
}
