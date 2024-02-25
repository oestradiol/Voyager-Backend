use crate::utils::ExpectError;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref HOSTNAME: String =
    std::env::var("HOSTNAME").unwrap_or_else(|_| "127.0.0.1".to_string());
  pub static ref PORT: String = std::env::var("PORT").unwrap_or_else(|_| "8765".to_string());
  pub static ref HOST_IP: String =
    std::env::var("HOST_IP").unwrap_or_else(|_| "host.docker.internal".to_string());
  pub static ref CLOUDFLARE_API_TOKEN: String = std::env::var("CLOUDFLARE_API_TOKEN")
    .expect_error(|e| format!("Failed to get Cloudflare API Token: {e}"));
  pub static ref CLOUDFLARE_ZONE: String = std::env::var("CLOUDFLARE_ZONE")
    .expect_error(|e| format!("Failed to get Cloudflare Zone: {e}"));
  pub static ref CLOUDFLARE_TARGET: String = std::env::var("CLOUDFLARE_TARGET")
    .expect_error(|e| format!("Failed to get Cloudflare Target: {e}"));
  pub static ref API_KEY: String =
    std::env::var("API_KEY").expect_error(|e| format!("Failed to get API Key: {e}"));
  pub static ref DISCORD_WEBHOOK: String = std::env::var("DISCORD_WEBHOOK")
    .expect_error(|e| format!("Failed to get Discord Webhook: {e}"));
  pub static ref GITHUB_ORG_NAME: String =
    std::env::var("GITHUB_ORG_NAME").unwrap_or_else(|_| "PinkCloudStudios".to_string());
  pub static ref GITHUB_PAT: String =
    std::env::var("GITHUB_PAT").expect_error(|e| format!("Failed to get GitHub PAT: {e}"));
  pub static ref DEPLOYMENTS_DIR: String =
    std::env::var("DEPLOYMENTS_DIR").unwrap_or_else(|_| "/var/opt/voyager/deployments".to_string());
  pub static ref STDOUT_LOG_SEVERITY: String =
    std::env::var("STDOUT_LOG_SEVERITY").unwrap_or_else(|_| "INFO".to_string());
  pub static ref LOG_DIRECTORY: String =
    std::env::var("LOG_DIRECTORY").unwrap_or_else(|_| "/var/log/voyager".to_string());
  pub static ref MONGO_CONN_STR: String = std::env::var("MONGO_CONN_STR")
    .expect_error(|e| format!("Failed to get MongoDB Connection String: {e}"));
  pub static ref MONGO_DB_NAME: String = std::env::var("MONGO_DB_NAME")
    .expect_error(|e| format!("Failed to get Mongo Database Name: {e}"));
  pub static ref DEVELOPMENT: bool = std::env::var("DEVELOPMENT").map_or_else(
    |_| false,
    |v| { matches!(v.to_lowercase().as_str(), "true") }
  );
}
