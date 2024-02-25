use axum::http::StatusCode;
use regex::Regex;
use tracing::{event, Level};

use crate::types::other::voyager_error::VoyagerError;

pub fn find_internal_port(docker_file_content: &str) -> Result<u16, VoyagerError> {
  event!(
    Level::DEBUG,
    "Retrieving internal docker port for docker file {}",
    docker_file_content
  );

  #[allow(clippy::unwrap_used)] // Should never fail since valid Regex
  Regex::new(r"EXPOSE (\d+)")
    .unwrap()
    .find(docker_file_content)
    .map_or_else(
      || Err(VoyagerError::parse_port()),
      |v| Ok(v.as_str().parse::<u16>().unwrap()),
    )
}

impl VoyagerError {
  fn parse_port() -> Self {
    Self::new(
      "Failed to parse internal port from Dockerfile".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      None,
    )
  }
}
