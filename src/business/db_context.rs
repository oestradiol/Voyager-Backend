use mongodb::{Client, options::ClientOptions};

use crate::{configs::environment::{MONGO_DB_NAME, MONGO_CONN_STR}, types::model::deployment::Deployment};

pub struct AppDbContext {
  pub deployments: mongodb::Collection<Deployment>,
}

impl AppDbContext {
  pub async fn init() -> Self {
    let client_options = ClientOptions::parse(&*MONGO_CONN_STR).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&*MONGO_DB_NAME);
    let deployments = db.collection("Deployments");

    Self {
      deployments,
    }
  }
}