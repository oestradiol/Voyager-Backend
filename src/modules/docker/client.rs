use futures::{executor, TryFutureExt};
use lazy_static::lazy_static;
use bollard::Docker;

use crate::{modules::docker::DOCKER_RUNTIME, Error};

lazy_static!(
  #[cfg(unix)]
  pub static ref DOCKER: Docker = executor::block_on(
      DOCKER_RUNTIME
        .spawn_blocking(|| Docker::connect_with_local_defaults()) // Spawns another thread to load Docker
        .map_err(Error::from) // Maps the JoinError type to our Error type
    )
    .map(|r| r.map_err(Error::from)) // Maps the Docker Error type to our Type
    .and_then(|f| f) // Flattens
    .expect("Failed to connect to Docker!");
);