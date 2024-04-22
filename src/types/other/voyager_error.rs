use std::{error::Error, fmt};

use axum::http::StatusCode;
use tracing::{event, Level};

use crate::utils::Error as OurErr;

#[derive(Debug)]
pub struct VoyagerError {
  pub message: String,
  pub status_code: StatusCode,
  pub source: Option<OurErr>,
}

impl fmt::Display for VoyagerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let message = format!(
      "Voyager Status Code: {}. {}.",
      self.status_code, self.message
    );

    if let Some(source) = &self.source {
      write!(f, "{message} Source Error: {source}")
    } else {
      write!(f, "{message}")
    }
  }
}

impl Error for VoyagerError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.source.as_deref().map(|s| s as _)
  }
}

impl VoyagerError {
  pub fn new(message: String, status_code: StatusCode, is_warn: bool, source: Option<OurErr>) -> Self {
    let result = Self {
      message,
      status_code,
      source,
    };
    
    if is_warn {
      event!(Level::WARN, "{result}");
    } else {
      event!(Level::ERROR, "{result}");
    };

    result
  }
}

