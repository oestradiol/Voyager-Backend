use crate::types::model::deployment::Deployment;
use serde::{Deserialize, Serialize};
use serde_json;
use tracing::{event, Level};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeployment {
  pub deployment: Option<Deployment>,
}

