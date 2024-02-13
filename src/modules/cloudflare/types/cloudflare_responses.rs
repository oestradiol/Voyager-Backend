use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CloudflareMessage {
  code: i32,
  message: String,
}
#[derive(Debug, Deserialize)]
pub struct CloudflareError {
  code: i32,
  message: String,
}
