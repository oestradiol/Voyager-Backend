use serde::Deserialize;

use std::{error::Error, fmt};

#[derive(Debug, Deserialize)]
pub struct CloudflareMessage {
  pub code: i32,
  pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CloudflareError {
  pub code: i32,
  pub message: String,
}

impl Error for CloudflareError {
  fn description(&self) -> &str {
    "CloudflareError"
  }
}

impl fmt::Display for CloudflareError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "A CloudflareError Response has occurred. Code: {}, Message: {}",
      self.code, self.message
    )
  }
}
