#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(
  clippy::complexity,
  clippy::pedantic,
  clippy::nursery,
  clippy::suspicious,
  clippy::perf,
  clippy::unwrap_used
)]

use axum::Router;
use dotenv::dotenv;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::level_filters::LevelFilter;
use tracing::{event, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

use crate::configs::environment::{HOSTNAME, LOG_DIRECTORY, PORT, STDOUT_LOG_SEVERITY};
use crate::controllers::ConfigureRoutes;
use crate::utils::ExpectError;

mod business;
mod configs;
mod controllers;
mod modules;
mod types;
mod utils;

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
  // .env
  dotenv().expect_error(|e| format!("Failed to load .env file: {e}"));

  // Logging
  init_logging();

  // Defining sockets
  let sock_host = HOSTNAME
    .parse::<Ipv4Addr>()
    .expect_error(|e| format!("Failed to parse HOST: {e}"));
  let port = PORT
    .parse::<u16>()
    .expect_error(|e| format!("Failed to parse PORT: {e}"));
  let sock_addr = SocketAddr::from((IpAddr::V4(sock_host), port));

  event!(
    Level::INFO,
    "Starting server at {}...",
    sock_addr.to_string()
  );

  let app = Router::new().configure_routes();

  let listener = tokio::net::TcpListener::bind(sock_addr)
    .await
    .expect_error(|e| format!("Failed to bind to socket! Error: {e}"));
  axum::serve(listener, app)
    .await
    .expect_error(|e| format!("Failed to start server! Error: {e}"));
}

fn init_logging() {
  std::env::set_var("RUST_SPANTRACE", "1");
  std::env::set_var("RUST_BACKTRACE", "1");
  std::env::set_var("RUST_LIB_BACKTRACE", "full");
  color_eyre::install().unwrap_or_default();

  let level_filter = match STDOUT_LOG_SEVERITY.as_str() {
    "TRACE" => LevelFilter::TRACE,
    "DEBUG" => LevelFilter::DEBUG,
    "WARN" => LevelFilter::WARN,
    "ERROR" => LevelFilter::ERROR,
    _ => LevelFilter::INFO,
  };

  let file_appender0 =
    tracing_appender::rolling::minutely(&*LOG_DIRECTORY, format!("Voyager.log"));
  let (non_blocking0, _guard) = tracing_appender::non_blocking(file_appender0);

  // let bollard_filter = EnvFilter::builder()
  //   .with_default_directive(LevelFilter::DEBUG.into())
  //   .from_env().map_err(|e| Error::from(e))?
  //   .add_directive("bollard=debug".parse().map_err(|e| Error::from(e))?);
  let stdout_log = tracing_subscriber::fmt::layer().pretty();
  let debug_log = tracing_subscriber::fmt::layer().with_writer(non_blocking0); 
  let layered = stdout_log.and_then(debug_log).with_filter(level_filter);

  tracing_subscriber::registry().with(layered).init();
}