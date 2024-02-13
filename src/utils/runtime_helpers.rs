use tokio::runtime::Runtime;
use tracing::{event, Level};

pub trait RuntimeSpawnHandled {
  async fn spawn_handled<F, T>(&self, task: &str, future: F) -> Option<T>
  where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static;
}
impl RuntimeSpawnHandled for Runtime {
  async fn spawn_handled<F, T>(&self, task: &str, future: F) -> Option<T>
  where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
  {
    self.spawn(future).await
      .map_or_else(
        |e| {
          event!(Level::ERROR, "Failed to complete task '{}'! Error: {e}", task);
          None
        },
        |f| Some(f)
    )
  }
}