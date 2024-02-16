pub mod deployments;
pub mod new_deployment;

use crate::utils::expect_error::ExpectError;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static! {
  pub static ref SERVICES_RUNTIME: Runtime =
    Runtime::new().expect_error(|e| format!("Failed to initialize Repositories Runtime: {e}"));
}
