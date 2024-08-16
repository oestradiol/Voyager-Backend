pub mod deployments;

use futures::executor;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, Client};
use tokio::runtime::Runtime;

use crate::{
  configs::environment::{MONGO_CONN_STR, MONGO_DB_NAME},
  types::model::deployment::Deployment,
  utils::ExpectError,
};

pub struct DbContext {
  pub deployments: mongodb::Collection<Deployment>,
}

impl DbContext {
  pub async fn init() -> Self {
    let client_options = ClientOptions::parse(&*MONGO_CONN_STR)
      .await
      .expect_error(|e| format!("Failed to parse MongoDB Connection String: {e}"));
    let client = Client::with_options(client_options)
      .expect_error(|e| format!("Failed to connect to MongoDB: {e}"));
    let db = client.database(&MONGO_DB_NAME);
    let deployments = db.collection("Deployments");

    Self { deployments }
  }
}

lazy_static! {
  pub static ref REPOSITORIES_RUNTIME: Runtime =
    Runtime::new().expect_error(|e| format!("Failed to initialize Repositories Runtime: {e}"));
  pub static ref DB_CONTEXT: DbContext = executor::block_on(DbContext::init());
}
