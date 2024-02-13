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
mod controllers;
mod configs;
mod utils;

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

  event!(Level::INFO, "Starting server at {}...", sock_addr.to_string());

  let app = Router::new()
    .route("/status", get(status));
  let listener = tokio::net::TcpListener::bind(sock_addr).await
    .map_err(|e| format!("Failed to bind to socket! Error: {e}")).unwrap();
  axum::serve(listener, app).await
    .map_err(|e| format!("Failed to start server! Error: {e}")).unwrap();
}

async fn status() -> &'static str {
  "Voyager is Up!"
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
