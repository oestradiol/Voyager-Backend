pub mod deployments;

use futures::executor;
use lazy_static::lazy_static;
use mongodb::{Client, options::ClientOptions};
use tokio::runtime::Runtime;

use crate::{configs::environment::{MONGO_DB_NAME, MONGO_CONN_STR}, types::model::deployment::Deployment};

pub struct AppDbContext {
  pub deployments: mongodb::Collection<Deployment>,
}

impl AppDbContext {
  pub async fn init() -> Self {
    let client_options = ClientOptions::parse(&*MONGO_CONN_STR).await.expect("Failed to parse connection string for MongoDB");
    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");
    let db = client.database(&*MONGO_DB_NAME);
    let deployments = db.collection("Deployments");

    Self {
      deployments,
    }
  }
}

lazy_static!(
  pub static ref REPOSITORIES_RUNTIME: Runtime = tokio::runtime::Runtime::new().unwrap();
  pub static ref APP_DB_CONTEXT: AppDbContext = executor::block_on(
      REPOSITORIES_RUNTIME.spawn(AppDbContext::init())
    ).expect("Failed to initialize database context");
);