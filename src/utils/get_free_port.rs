use std::net::TcpListener;

use axum::http::StatusCode;
use tracing::{event, Level};

use crate::{
  configs::environment::HOST_IP, types::other::voyager_error::VoyagerError, utils::Error,
};

pub fn get_free_port() -> Result<u16, VoyagerError> {
  event!(Level::INFO, "Attempting to get free port");
  let port = _get_free_port().map_err(VoyagerError::get_free_port)?;

  event!(Level::INFO, "Successfully got free port: {port}");
  Ok(port)
}

fn _get_free_port() -> Result<u16, Error> {
  Ok(
    TcpListener::bind(format!("{}:0", *HOST_IP))?
      .local_addr()?
      .port(),
  )
}

impl VoyagerError {
  fn get_free_port(e: Error) -> Self {
    Self::new(
      "Failed to get free port".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
