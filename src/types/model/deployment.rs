use mongodb::bson::{self};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
  Preview,
  Production,
}

impl fmt::Display for Mode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Preview => write!(f, "Preview"),
      Self::Production => write!(f, "Production"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
  pub _id: bson::oid::ObjectId,
  pub container_id: String,
  pub dns_record_id: String,
  pub container_name: String,
  pub image_id: String,
  // pub port: u16,
  pub mode: Mode,
  pub host: String,
  pub repo_url: String,
  pub branch: String,
}
