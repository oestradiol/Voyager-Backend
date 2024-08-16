// use std::net::TcpListener;

// use axum::http::StatusCode;
// use tracing::{event, Level};

// use crate::{
//   configs::environment::HOSTNAME, types::other::voyager_error::VoyagerError, utils::Error,
// };

// pub fn get_free_port() -> Result<u16, VoyagerError> {
//   let fun = || {
//     Ok::<u16, Error>(
//       TcpListener::bind(format!("{}:0", *HOSTNAME))?
//         .local_addr()?
//         .port(),
//     )
//   };

//   event!(Level::INFO, "Attempting to get free port");
//   let port = fun().map_err(VoyagerError::get_free_port)?;

//   event!(Level::INFO, "Successfully got free port: {port}");
//   Ok(port)
// }

// impl VoyagerError {
//   fn get_free_port(e: Error) -> Self {
//     Self::new(
//       "Failed to get free port".to_string(),
//       StatusCode::INTERNAL_SERVER_ERROR,
//       false,
//       Some(e),
//     )
//   }
// }
