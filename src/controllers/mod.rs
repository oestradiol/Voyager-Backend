pub mod deployments;

use axum::{
  extract::Request, http::StatusCode, middleware::{self, Next}, response::{IntoResponse, Json, Response}, routing::{delete, get, post}, Router
};
use serde::Serialize;

use crate::{configs::environment::API_KEY, types::view::logs::Logs};

pub trait ConfigureRoutes {
  fn configure_routes(self) -> Self;
}
impl ConfigureRoutes for Router {
  fn configure_routes(self) -> Self {
    self.nest(
      "/api/v1",
      Self::new()
        .route("/status", get(status))
        .nest(
          "/deployments",
          Self::new()
          .route("/", post(deployments::create))
          .route("/", get(deployments::list))
          .route("/:id", get(deployments::get))
          .route("/:id", delete(deployments::delete))
          .route("/:id/logs", get(deployments::get_logs))
          .layer(middleware::from_fn(authorization_middleware)),
        ),
    )
  }
}

async fn status() -> impl IntoResponse {
  Json(BasicResponse {
    logs: Logs {
      message: "Voyager is Up and Running!".to_string(),
      errors: vec![]
    }
  })
}

async fn authorization_middleware(
  request: Request,
  next: Next,
) -> Response {
  let api_key = request.headers().get("X-Api-Key");
  #[allow(clippy::unwrap_used)] // Should never fail
  if api_key.is_none() || api_key.unwrap() != &*API_KEY {
    let mut response = Json(BasicResponse {
      logs: Logs {
        message: "Unauthorized".to_string(),
        errors: vec!["Invalid API Key".to_string()]
      }
    }).into_response();
    *response.status_mut() = StatusCode::UNAUTHORIZED;
    response
  } else {
    next.run(request).await
  }
}

#[derive(Serialize)]
struct BasicResponse {
  logs: Logs,
}