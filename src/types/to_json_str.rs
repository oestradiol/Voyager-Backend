use axum::http::StatusCode;
use serde::Serialize;
use tracing::{event, Level};

use crate::utils::Error;

use super::other::voyager_error::VoyagerError;

pub fn to_json_str(obj: &(impl Serialize + std::fmt::Debug)) -> Result<String, VoyagerError> {
  serde_json::to_string(obj).map_err(|e| VoyagerError::to_json_str(obj, Box::new(e)))
}

impl VoyagerError {
  pub fn to_json_str(obj: &(impl Serialize + std::fmt::Debug), e: Error) -> Self {
    let message = format!("Failed to convert {obj:?} to json string! Error: {e}");
    event!(Level::ERROR, message);
    Self {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: Some(e),
    }
  }
}
