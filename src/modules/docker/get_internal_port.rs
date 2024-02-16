use regex::Regex;
use tracing::{event, Level};

pub fn find_internal_port(docker_file_content: &str) -> Option<u16> {
  event!(
    Level::DEBUG,
    "Retrieving internal docker port for docker file {}",
    docker_file_content
  );

  #[allow(clippy::unwrap_used)] // Should never fail since valid Regex
  Regex::new(r"EXPOSE (\d+)")
    .unwrap()
    .find(docker_file_content)
    .map_or_else(
      || {
        event!(
          Level::ERROR,
          "Failed to parse internal port from docker file!"
        );
        None
      },
      |v| Some(v.as_str().parse::<u16>().unwrap()),
    )
}
