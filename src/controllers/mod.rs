use axum::Router;

trait ConfigureRoutes {
  fn configure_routes(self) -> Self;
}
impl ConfigureRoutes for Router {
  fn configure_routes(self) -> Self {
    self
      .nest("/api/v1", Self::new()
        .nest("/deployments", Self::new()
            // .route("/", get(business::services::deployments::list))
            // .route("/", post(business::services::deployments::create))
            // .route("/:deploymentId", post(business::services::deployments::get))
            // .route("/:deploymentId", post(business::services::deployments::delete))
            // .route("/:deploymentId/logs", post(business::services::deployments::get_logs))
        )
      )
  }
}
