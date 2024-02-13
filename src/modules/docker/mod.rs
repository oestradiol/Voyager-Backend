mod client;

pub mod build_image;
pub mod create_and_start_container;
pub mod delete_container;
pub mod find_internal_port;
pub mod get_logs;
pub mod is_container_running;
pub mod restart_container;
pub mod stop_container_and_delete;
pub mod stop_container;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static!(
  pub static ref DOCKER_RUNTIME: Runtime = Runtime::new().unwrap();
);