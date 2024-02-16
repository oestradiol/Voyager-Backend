use std::net::TcpListener;

use crate::configs::environment::HOST_IP;

pub fn get_free_port() -> Result<u16, std::io::Error> {
  Ok(TcpListener::bind(format!("{}:0", *HOST_IP))?.local_addr()?.port())
}
