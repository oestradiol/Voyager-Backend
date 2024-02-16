use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CloudflareMessage {
  pub code: i32,
  pub message: String,
}
#[derive(Debug, Deserialize)]
pub struct CloudflareError {
  pub code: i32,
  pub message: String,
}
