use axum::http::StatusCode;
use tokio::runtime::Runtime;

use super::Error;
use crate::types::other::voyager_error::VoyagerError;

pub trait RuntimeSpawnHandled {
  async fn spawn_handled<F, T>(&self, task: &str, future: F) -> Result<T, VoyagerError>
  where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static;
}

impl RuntimeSpawnHandled for Runtime {
  async fn spawn_handled<F, T>(&self, task: &str, future: F) -> Result<T, VoyagerError>
  where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
  {
    self
      .spawn(future)
      .await
      .map_or_else(|e| Err(VoyagerError::spawn(task, Box::new(e))), |f| Ok(f))
  }
}

impl VoyagerError {
  pub fn spawn(task: &str, e: Error) -> Self {
    Self::new(
      format!("Failed to complete task '{task}'"),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
