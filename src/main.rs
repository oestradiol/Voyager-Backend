#!forbid(unsafe_code)

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::{event, Level};
use axum::{routing::get, Router};
use dotenv::dotenv;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

use crate::{configs::environment::{HOSTNAME, PORT, STDOUT_LOG_SEVERITY, LOG_DIRECTORY}, types::view::{create_deployment::CreateDeployment, logs::Logs}};

mod business;
mod modules;
mod types;
mod utils;
mod controllers;
mod configs;

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
  let time: DateTime<Utc> = SystemTime::now().into();
  let time_str = time.to_rfc3339();

  // .env
  dotenv().expect("Failed to load .env file");

  // Logging
  std::env::set_var("RUST_LOG", &*STDOUT_LOG_SEVERITY);
  std::env::set_var("RUST_BACKTRACE", "1");
  std::env::set_var("RUST_LIB_BACKTRACE", "full");
  tracing_subscriber::fmt::init();
  color_eyre::install().unwrap_or_default();
  let file_appender0 = tracing_appender::rolling::never(&*LOG_DIRECTORY, format!("{time_str}.log"));
  let file_appender1 = tracing_appender::rolling::never(&*LOG_DIRECTORY, "latest.log");
  let (non_blocking0, _guard) = tracing_appender::non_blocking(file_appender0);
  let (non_blocking1, _guard) = tracing_appender::non_blocking(file_appender1);
  tracing_subscriber::fmt()
    .with_writer(non_blocking0)
    .init();
  tracing_subscriber::fmt()
    .with_writer(non_blocking1)
    .init();


  // Defining sockets
  let sock_host = HOSTNAME
    .parse::<Ipv4Addr>()
    .expect("Failed to parse HOST");
  let port = PORT
    .parse::<u16>()
    .expect("Failed to parse PORT");
  let sock_addr = SocketAddr::from((
    IpAddr::V4(sock_host),
    port,
  ));

  event!(Level::INFO, "Starting server...");

  let app = Router::new()
    .route("/status", get(status));
  let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn status() -> &'static str {
  "Voyager is Up!"
}
