mod build_image;
pub use build_image::*;

mod create_container;
pub use create_container::*;

mod delete_container;
pub use delete_container::*;

mod delete_image;
pub use delete_image::*;

mod get_internal_port;
pub use get_internal_port::*;

mod get_logs;
pub use get_logs::*;

mod is_container_running;
pub use is_container_running::*;

mod restart_container;

mod start_container;
pub use start_container::*;

mod stop_container;
pub use stop_container::*;

use crate::utils::Error;
use crate::utils::ExpectError;

use bollard::Docker;
use futures::{executor, TryFutureExt};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static!(
  #[cfg(unix)]
  pub static ref DOCKER: Docker = executor::block_on(
      DOCKER_RUNTIME
        .spawn_blocking(Docker::connect_with_local_defaults) // Spawns another thread to load Docker
        .map_err(Error::from) // Maps the JoinError type to our Error type
    )
    .map(|r| r.map_err(Error::from)) // Maps the Docker Error type to our Type
    .and_then(|f| f) // Flattens
    .expect_error(|e| format!("Failed to connect to Docker! Error: {e}"));
);

lazy_static! {
  pub static ref DOCKER_RUNTIME: Runtime =
    Runtime::new().expect_error(|e| format!("Failed to initialize Docker Runtime: {e}"));
}
