use lazy_static::lazy_static;
use bollard::Docker;

lazy_static! {
  #[cfg(unix)]
  pub static ref DOCKER: Docker = Docker::connect_with_socket_defaults().expect("Failed to connect to Docker");
}