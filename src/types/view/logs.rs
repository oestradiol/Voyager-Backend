use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Logs {
  pub message: String,
  pub errors: Vec<String>,
}