use std::net::TcpListener;

use tracing::{event, Level};

use crate::{configs::environment::HOST_IP, utils::Error};

pub fn get_free_port() -> Option<u16> {
  event!(Level::INFO, "Attempting to get free port");
  match _get_free_port() {
    Ok(port) => {
      event!(Level::INFO, "Succcessfully Got free port: {}", port);
      Some(port)
    }
    Err(e) => {
      event!(Level::ERROR, "Failed to get free port: {}", e);
      None
    }
  }
}

fn _get_free_port() -> Result<u16, Error> {
  Ok(
    TcpListener::bind(format!("{}:0", *HOST_IP))?
      .local_addr()?
      .port(),
  )
}
