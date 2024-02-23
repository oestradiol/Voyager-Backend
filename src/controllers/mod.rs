mod deployments;

use axum::{
  routing::{delete, get, post},
  Router,
};

trait ConfigureRoutes {
  fn configure_routes(self) -> Self;
}
impl ConfigureRoutes for Router {
  fn configure_routes(self) -> Self {
    self.nest(
      "/api/v1",
      Self::new().nest(
        "/deployments",
        Self::new()
          .route("/", get(deployments::list))
          .route("/", post(deployments::create))
          .route("/:deploymentId", get(deployments::get))
          .route("/:deploymentId", delete(deployments::delete))
          .route("/:deploymentId/logs", get(deployments::get_logs)),
      ),
    )
  }
}
