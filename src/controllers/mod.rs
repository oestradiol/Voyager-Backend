pub mod deployments;

use axum::{
  routing::{delete, get, post},
  Router,
};

pub trait ConfigureRoutes {
  fn configure_routes(self) -> Self;
}
impl ConfigureRoutes for Router {
  fn configure_routes(self) -> Self {
    self.nest(
      "/api/v1",
      Self::new().nest(
        "/deployments",
        Self::new()
        .route("/", post(deployments::create))
        .route("/", get(deployments::list))
        .route("/:id", get(deployments::get))
        .route("/:id", delete(deployments::delete))
        .route("/:id/logs", get(deployments::get_logs)),
      ),
    )
  }
}