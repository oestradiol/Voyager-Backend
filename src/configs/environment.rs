use std::str::FromStr;

use crate::utils::ExpectError;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref HOSTNAME: String = var_opt("HOSTNAME").unwrap_or_else(|| "127.0.0.1".to_string());
  pub static ref PORT: String = var_opt("PORT").unwrap_or_else(|| "8765".to_string());
  pub static ref HOST_IP: String =
    var_opt("HOST_IP").unwrap_or_else(|| "host.docker.internal".to_string());
  pub static ref CLOUDFLARE_API_TOKEN: String = var("CLOUDFLARE_API_TOKEN");
  pub static ref CLOUDFLARE_ZONE: String = var("CLOUDFLARE_ZONE");
  pub static ref CLOUDFLARE_TARGET: String = var("CLOUDFLARE_TARGET");
  pub static ref API_KEY: String = var("API_KEY");
  pub static ref DISCORD_WEBHOOK: String = var("DISCORD_WEBHOOK");
  pub static ref GITHUB_ORG_NAME: String =
    var_opt("GITHUB_ORG_NAME").unwrap_or_else(|| "PinkCloudStudios".to_string());
  pub static ref GITHUB_PAT: String = var("GITHUB_PAT");
  pub static ref DEPLOYMENTS_DIR: String =
    var_opt("DEPLOYMENTS_DIR").unwrap_or_else(|| "/var/opt/voyager/deployments".to_string());
  pub static ref STDOUT_LOG_SEVERITY: String =
    var_opt("STDOUT_LOG_SEVERITY").unwrap_or_else(|| "INFO".to_string());
  pub static ref LOG_DIRECTORY: String =
    var_opt("LOG_DIRECTORY").unwrap_or_else(|| "/var/log/voyager".to_string());
  pub static ref MONGO_CONN_STR: String = var("MONGO_CONN_STR");
  pub static ref MONGO_DB_NAME: String = var("MONGO_DB_NAME");
  pub static ref DEVELOPMENT: bool = var_opt("DEVELOPMENT").unwrap_or(false);
}

fn var<T: FromStr>(name: &'static str) -> T {
  std::env::var(name)
    .unwrap_or_else(|_| panic!("Couldn't find env variable {name}"))
    .parse::<T>()
    .ok()
    .unwrap_or_else(|| panic!("Couldn't parse env variable {name}"))
}

fn var_opt<T: FromStr>(name: &'static str) -> Option<T> {
  std::env::var(name).ok()?.parse::<T>().ok()
}
