#![forbid(unsafe_code)]
#![deny(warnings)]
#![allow(unused)] // Temporarily here while we are working on the project
#![warn(
  clippy::complexity,
  clippy::pedantic,
  clippy::nursery,
  clippy::suspicious,
  clippy::perf,
  clippy::unwrap_used
)]

use axum::{routing::get, Router};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::SystemTime;
use tracing::{event, Level};

use crate::configs::environment::{HOSTNAME, LOG_DIRECTORY, PORT, STDOUT_LOG_SEVERITY};
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
  let time: DateTime<Utc> = SystemTime::now().into();
  let time_str = time.to_rfc3339();

  // .env
  dotenv().expect_error(|e| format!("Failed to load .env file: {e}"));

  // Logging
  init_logging(&time_str);

  // Defining sockets
  let sock_host = HOSTNAME
    .parse::<Ipv4Addr>()
    .expect_error(|e| format!("Failed to parse HOST {e}"));
  let port = PORT
    .parse::<u16>()
    .expect_error(|e| format!("Failed to parse PORT: {e}"));
  let sock_addr = SocketAddr::from((IpAddr::V4(sock_host), port));

  event!(
    Level::INFO,
    "Starting server at {}...",
    sock_addr.to_string()
  );

  let app = Router::new().route("/status", get(status));
  let listener = tokio::net::TcpListener::bind(sock_addr)
    .await
    .expect_error(|e| format!("Failed to bind to socket! Error: {e}"));
  axum::serve(listener, app)
    .await
    .expect_error(|e| format!("Failed to start server! Error: {e}"));
}

async fn status() -> &'static str {
  "Voyager is Up!"
}

fn init_logging(time_str: &str) {
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
    tracing_appender::rolling::minutely(&*LOG_DIRECTORY, format!("{time_str}.log"));
  let (non_blocking0, _guard) = tracing_appender::non_blocking(file_appender0);

  let stdout_log = tracing_subscriber::fmt::layer().pretty();
  let debug_log = tracing_subscriber::fmt::layer().with_writer(non_blocking0);

  tracing_subscriber::registry()
    .with(stdout_log.and_then(debug_log).with_filter(level_filter))
    .init();
}

//// TODO:
// fun Application.init() {
//     val globalExceptionHandler =
//         Thread.UncaughtExceptionHandler { thread, err ->
//             try {
//                 log("Uncaught exception in thread ${thread.name}:", LogType.FATAL)
//                 log(err)
//
//                 Logger.cleanup()
//             } catch (err2: Exception) {
//                 err.printStackTrace()
//                 err2.printStackTrace()
//             }
//         }
//
//     Thread.setDefaultUncaughtExceptionHandler(globalExceptionHandler)
//
//     Runtime.getRuntime().addShutdownHook(
//         object : Thread() {
//             override fun run() {
//                 try {
//                     log("Shutdown hook called, cleaning up..", LogType.WARN)
//
//                     Logger.cleanup()
//                 } catch (err: Exception) {
//                     err.printStackTrace()
//                 }
//             }
//         }
//     )
//
//
//     log("Registering call interceptors..", LogType.INFO)
//     // install(HttpsRedirect)
//
//     intercept(ApplicationCallPipeline.Call) {
//         val apiKey = call.request.header("X-API-Key")
//
//         if (apiKey == null || apiKey != VOYAGER_CONFIG.apiKey) {
//
//             // Preventing log spam
//             if (call.request.origin.remoteAddress != "127.0.0.1") {
//                 log("User tried to connect with invalid API Key, IP address is:  ${call.request.origin.remoteAddress}", LogType.WARN)
//             }
//
//             call.respond(
//                 HttpStatusCode.Unauthorized,
//                 "Invalid API Key"
//             )
//             return@intercept finish()
//         }
//     }
//
//     configureDeploymentApi()
// }
