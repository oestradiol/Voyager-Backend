use mongodb::{bson::{doc, Bson}, Cursor};
use tracing::{event, Level};
use crate::{
  business::repositories::{APP_DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::model::deployment::Deployment, Error,
  utils::runtime_helpers::RuntimeSpawnHandled
};

pub async fn find_by_id(id: String) -> Option<Deployment> {
  event!(Level::DEBUG, "Finding deployment with id {}", &id);

  let id_clone = id.clone();
  let future = 
    async move {
      let result = APP_DB_CONTEXT.deployments
        .find_one(doc! { "_id": &id }, None).await;

      let result = result
        .map_err(Error::from) // MongoDB Error
        .map(|d| d.ok_or(Error::from("Deployment not found"))) // 'None' Error
        .and_then(|inner| inner); // Flatten

      result
    };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::find_by_id", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to find deployment with id {}: {}", id_clone, e);
      None
    }, |d| Some(d))
  }).and_then(|d| d)
}

pub async fn find_by_host(host: String) -> Option<Deployment> {
  event!(Level::DEBUG, "Finding deployment by host {}", &host);

  let host_clone = host.clone();
  let future = 
    async move {
      let result = APP_DB_CONTEXT.deployments
        .find_one(doc! { "host": &host }, None).await;

      let result = result
        .map_err(Error::from) // MongoDB Error
        .map(|d| d.ok_or(Error::from("Deployment not found"))) // 'None' Error
        .and_then(|inner| inner); // Flatten

      result
    };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::find_by_host", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to find deployment with host {}: {}", host_clone, e);
      None
    }, |d| Some(d))
  }).and_then(|d| d)
}

pub async fn retrieve_all() -> Option<Cursor<Deployment>> {
  event!(Level::DEBUG, "Retrieving ALL deployments...");

  let future = 
    async move {
      let result = APP_DB_CONTEXT.deployments
        .find(doc! {}, None).await;

      result.map_err(Error::from) // MongoDB Error
    };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::retrieve_all", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
      None
    }, |c| Some(c))
  }).and_then(|c| c)
}

pub async fn retrieve_all_by_repo_url_and_branch(repo_url: String, branch: Option<String>) -> Option<Cursor<Deployment>> {
  let repo_and_branch = branch.clone().map_or("".to_string(), |b| format!("@{b}"));
  let repo_and_branch = format!("{}{}", repo_url, repo_and_branch);
  event!(Level::DEBUG, "Retrieving deployments from {repo_and_branch}");

  let future = 
    async move {
      let result = APP_DB_CONTEXT.deployments
        .find(doc! {"repo_url": repo_url, "branch": branch}, None).await;

      result.map_err(Error::from) // MongoDB Error
    };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::retrieve_all_by_repo_url_and_branch", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments for {repo_and_branch}: {}", e);
      None
    }, |c| Some(c))
  }).and_then(|c| c)
}

pub async fn save(deployment: Deployment) -> Option<Bson> {
  event!(Level::DEBUG, "Retrieving ALL deployments...");

  let future = 
    async move {
      let result = APP_DB_CONTEXT.deployments
        .insert_one(deployment, None).await;

      result.map_err(Error::from) // MongoDB Error
  };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::save", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
      None
    }, |f| Some(f.inserted_id))
  }).and_then(|id| id)
}

pub async fn delete(id: String) -> bool {
  event!(Level::DEBUG, "Retrieving ALL deployments...");

  let future = 
    async move {
      let result = APP_DB_CONTEXT.deployments
        .delete_one(doc! {"_id": id}, None).await;

      result.map_err(Error::from) // MongoDB Error
  };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::delete", future).await;


  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
      false
    }, |d| d.deleted_count > 0)
  }).map_or(false, |f| f)
}