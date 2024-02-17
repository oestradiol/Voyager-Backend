use std::net::TcpListener;

use crate::{configs::environment::HOST_IP, utils::Error};

pub fn get_free_port() -> Result<u16, Error> {
  Ok(
    TcpListener::bind(format!("{}:0", *HOST_IP))?
      .local_addr()?
      .port(),
  )
}
