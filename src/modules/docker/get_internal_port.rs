use tracing::{event, Level};

fn find_internal_port(docker_file_content: &str) -> Option<u16> {
  event!(Level::DEBUG, "Retrieving internal docker port for docker file {}", docker_file_content);

  docker_file_content
    .split("EXPOSE ")
    .collect::<Vec<&str>>()[1]
    .split('\n')
    .collect::<Vec<&str>>()[0]
    .parse::<u16>()
    .map_or_else(|e| {
      event!(Level::ERROR, "Failed to parse internal port from docker file! Error: {}", e);
      None
    }, Some)
}
