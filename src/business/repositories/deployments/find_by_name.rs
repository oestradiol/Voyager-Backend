use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::model::deployment::Deployment,
  utils::runtime_helpers::RuntimeSpawnHandled,
  utils::Error,
};
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn find_by_name(name: String) -> Option<Deployment> {
  event!(Level::DEBUG, "Finding deployment with name {}", &name);

  let name_clone = name.clone();
  let future = async move {
    let result = DB_CONTEXT
      .deployments
      .find_one(doc! { "name": &name }, None)
      .await;

    result
      .map_err(Error::from) // MongoDB Error
      .map(|d| d.ok_or_else(|| Error::from("Deployment not found"))) // 'None' Error
      .and_then(|inner| inner) // Flatten
  };

  let result = REPOSITORIES_RUNTIME
    .spawn_handled("repositories::deployments::find_by_name", future)
    .await;

  result
    .map(|r| {
      r.map_or_else(
        |e| {
          event!(
            Level::ERROR,
            "Failed to find deployment with name {}: {}",
            name_clone,
            e
          );
          None
        },
        Some,
      )
    })
    .and_then(|d| d)
}
