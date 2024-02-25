use std::{error::Error, fmt};

use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::utils::Error as OurErr;

#[derive(Debug)]
pub struct VoyagerError {
  pub message: String,
  pub status_code: StatusCode,
  pub source: Option<OurErr>,
}

impl fmt::Display for VoyagerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Error: {} (Code: {})", self.message, self.status_code)
  }
}

impl Error for VoyagerError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.source.as_deref().map(|s| s as _)
  }
}
