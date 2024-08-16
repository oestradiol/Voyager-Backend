use axum::http::StatusCode;
use regex::Regex;
use tracing::{event, Level};

use crate::{types::other::voyager_error::VoyagerError, utils::Error};

pub fn find_internal_port(docker_file_content: &str) -> Result<u16, VoyagerError> {
  event!(
    Level::DEBUG,
    "Retrieving internal docker port from Dockerfile"
  );

  #[allow(clippy::unwrap_used)] // Should never fail since valid Regex
  Regex::new(r"EXPOSE (\d+)")
    .unwrap()
    .captures_iter(docker_file_content)
    .map(|c| c[1].to_string())
    .next()
    .ok_or_else(|| VoyagerError::parse_port(None))?
    .parse::<u16>()
    .map_err(|e| VoyagerError::parse_port(Some(Box::new(e))))
}

impl VoyagerError {
  fn parse_port(e: Option<Error>) -> Self {
    Self::new(
      "Failed to parse internal port from Dockerfile".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      e,
    )
  }
}
