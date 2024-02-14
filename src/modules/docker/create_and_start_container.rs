use std::collections::HashMap;

use bollard::{
  container::{Config, CreateContainerOptions},
  service::{HostConfig, PortBinding},
};

use super::DOCKER;

pub async fn create_and_start_container(
  name: String,
  port: u16,
  internal_port: u16,
  docker_image: String,
) -> Result<String, bollard::errors::Error> {
  let options = Some(CreateContainerOptions {
    name: name,
    platform: Some("linux/amd64".to_string()),
  });

  let host_config = HostConfig {
    port_bindings: Some(HashMap::from([(
      internal_port.to_string(),
      Some(vec![PortBinding {
        host_ip: Some("127.0.0.1".to_string()),
        host_port: Some(port.to_string()),
      }]),
    )])),
    ..Default::default()
  };

  let config = Config {
    image: Some(docker_image),
    host_config: Some(host_config),
    ..Default::default()
  };

  DOCKER
    .create_container(options, config)
    .await
    .map(|res| res.id)
}

//     suspend fun createAndStartContainer(
//         name: String,
//         port: Int,
//         internalPort: Int,
//         dockerImage: String
//     ): Result<String> = coroutineScope {
//         log("Creating and starting container with name: $name, port: $port, internal port: $internalPort, docker image: $dockerImage", LogType.INFO)
//         val context = newSingleThreadContext("DockerThread-${dockerImage.hashCode()}")
//
//         val id: String
//
//         try {
//             id = async(context) {
//                 log("Building docker id and blocking this thread..", LogType.DEBUG)
//                 val idIn =
//                     dockerClient
//                         .createContainerCmd(dockerImage)
//                         .withName(name)
//                         // expose these ports inside the container
//                         .withExposedPorts(
//                             ExposedPort.tcp(internalPort)
//                         )
//                         .withHostConfig(
//                             HostConfig.newHostConfig()
//                                 .withPortBindings(
//                                     // map the ${internalPort} port inside the container to the ${port} port on the host
//                                     PortBinding(
//                                         Ports.Binding.bindPort(port),
//                                         ExposedPort.tcp(internalPort)
//                                     )
//                                 )
//                         )
//                         .exec()
//                         .id // the id of the container that was created. (this container is not running yet)
//
//                 log("Container built with id $idIn", LogType.DEBUG)
//                 log("Starting container with id $idIn", LogType.DEBUG)
//
//                 dockerClient
//                     .startContainerCmd(idIn)
//                     .exec()
//
//                 log("Container with id $idIn started", LogType.DEBUG)
//
//                 return@async idIn
//             }.await()
//
//         } catch (err: Exception) {
//             log("Error while creating and starting container:", LogType.ERROR)
//             log(err)
//             context.close()
//
//             return@coroutineScope Result.failure(err)
//         } finally {
//             context.close()
//         }
//
//         return@coroutineScope Result.success(id)
//
//     }

