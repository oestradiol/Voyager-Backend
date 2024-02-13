pub mod deployments;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static!(
  pub static ref SERVICES_RUNTIME: Runtime =  Runtime::new().unwrap();
);