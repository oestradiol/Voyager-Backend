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
use tracing_appender::non_blocking::WorkerGuard;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
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

  // Logging - The variables are needed for the lifetime of the program
  let (_log_guard_0, _log_guard_1) = init_logging().expect_error(|e| format!("Failed to initialize logging: {e}"));

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

fn init_logging() -> Result<(WorkerGuard, WorkerGuard), io::Error> {
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

  let dir = Path::new(&*LOG_DIRECTORY).canonicalize()?;
  let file_appender =
    tracing_appender::rolling::daily(dir, format!("Voyager.log"));
  let (non_blocking_file, guard0) = tracing_appender::non_blocking(file_appender);
  let (non_blocking_stdout, guard1) = tracing_appender::non_blocking(std::io::stdout());
  
  let file_log = tracing_subscriber::fmt::layer().with_writer(non_blocking_file);
  let stdout_log = tracing_subscriber::fmt::layer().pretty().with_writer(non_blocking_stdout);

  let layered = stdout_log.and_then(file_log).with_filter(level_filter);

  tracing_subscriber::registry().with(layered).init();

  Ok((guard0, guard1))
}
