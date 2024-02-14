
use lazy_static::lazy_static;

lazy_static! {
  pub static ref HOSTNAME: String = std::env::var("HOSTNAME").unwrap_or_else(|_| "127.0.0.1".to_string());
  pub static ref PORT: String = std::env::var("PORT").unwrap_or_else(|_| "8765".to_string());
  pub static ref HOST_IP: String = std::env::var("HOST_IP").unwrap_or_else(|_| "host.docker.internal".to_string());

  pub static ref CLOUDFLARE_API_TOKEN: String = std::env::var("CLOUDFLARE_API_TOKEN").unwrap();
  pub static ref CLOUDFLARE_ZONE: String = std::env::var("CLOUDFLARE_ZONE").unwrap();
  pub static ref CLOUDFLARE_TARGET: String = std::env::var("CLOUDFLARE_TARGET").unwrap();
  pub static ref API_KEY: String = std::env::var("API_KEY").unwrap();
  pub static ref DISCORD_WEBHOOK: String = std::env::var("DISCORD_WEBHOOK").unwrap();

  pub static ref GITHUB_ORG_NAME: String = std::env::var("GITHUB_ORG_NAME").unwrap_or_else(|_| "PinkCloudStudios".to_string());
  pub static ref DEPLOYMENTS_DIR: String = std::env::var("DEPLOYMENTS_DIR").unwrap_or_else(|_| "/var/opt/voyager/deployments".to_string());
  pub static ref STDOUT_LOG_SEVERITY: String = std::env::var("STDOUT_LOG_SEVERITY").unwrap_or_else(|_| "INFO".to_string());
  pub static ref LOG_DIRECTORY: String = std::env::var("LOG_DIRECTORY").unwrap_or_else(|_| "/var/log/voyager".to_string());

  pub static ref MONGO_CONN_STR: String = std::env::var("MONGO_CONN_STR").unwrap();
  pub static ref MONGO_DB_NAME: String = std::env::var("MONGO_DB_NAME").unwrap();

  pub static ref DEVELOPMENT: bool = std::env::var("DEVELOPMENT")
    .map(|v| {
      matches!(v.as_str(), "true")
    }).unwrap();
}
