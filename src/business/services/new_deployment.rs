use std::fs;
use std::path::Path;

use crate::configs::environment::HOST_IP;
use crate::modules::{cloudflare, traefik};
use crate::types::model::deployment;
use crate::utils::get_free_port;
use crate::{business::repositories::deployments, modules::docker};
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
  types::model::deployment::{Deployment, Mode},
  utils::Error,
};

// Result<Result<T, Err>, Exc>
// Result<T, Result<Err, Exc>>

pub async fn new_deployment(
  dockerfile: &Path,
  host: &String,
  mode: &deployment::Mode,
  directory: &String,
  repo_url: &String,
  branch: &String,
) -> Result<Deployment, Vec<Error>> {
  event!(Level::INFO, "Creating deployment with host {host}, Dockerfile {:?}, mode {mode}, directory {directory}, repo_url {repo_url}, branch {branch}", dockerfile);

  let id = Uuid::new_v4().to_string();

  let cloudflare_result: Result<String, Vec<Error>> =
    cloudflare::add_dns_record(host, &*HOST_IP, mode)
      .await
      .map_err(|errs| {
        errs
          .iter()
          .map(|err| Error::from(format!("CloudflareError {}: {}", err.code, err.message)))
          .collect()
      });
  let cloudflare_id;

  match cloudflare_result {
    Ok(id) => cloudflare_id = id,
    Err(errs) => return Err(errs),
  }

  let dockerfile_contents_result = fs::read_to_string(dockerfile);
  let dockerfile_contents;

  match dockerfile_contents_result {
    Ok(contents) => dockerfile_contents = contents,
    Err(err) => return Err(vec![err.into()]),
  }

  let internal_port_result = docker::find_internal_port(dockerfile_contents.as_str());
  let internal_port;

  match internal_port_result {
    Some(port) => internal_port = port,
    None => {
      return Err(vec![Error::from("Could not find container internal port")]);
    }
  }

  let name = host.replace(".", "");

  let traefik_labels = traefik::gen_traefik_labels(&name, &host, internal_port);

  let docker_image_result = docker::build_image(dockerfile, &traefik_labels, None).await;
  let docker_image;

  match docker_image_result {
    Ok(image) => docker_image = image,
    Err(err) => {
      return Err(vec![err]);
    }
  }

  let free_port_result = get_free_port();
  let free_port;

  match free_port_result {
    Ok(port) => free_port = port,
    Err(err) => {
      return Err(vec![err]);
    }
  }

  let container_id_result =
    docker::create_container(name.clone(), free_port, internal_port, docker_image).await;
  let container_id;

  match container_id_result {
    Ok(id) => container_id = id,
    Err(err) => {}
  }

  ()
}

//         suspend fun new(
//             dockerFile: File,
//             host: String,
//             mode: DeploymentMode,
//             directory: String,
//             repoUrl: String,
//             branch: String
//         ): Either<String, Deployment> {
//             return withContext(context) {
//                 log("Creating deployment with host $host, dockerFile: $dockerFile, host: $host, mode $mode", LogType.INFO)
//
//                 val id = UUID
//                     .randomUUID()
//                     .toString()
//
//
//                 log("Sending add DNS record request to Cloudflare..", LogType.DEBUG)
//                 val cloudflareResult = async { CloudflareManager.INSTANCE.addDnsRecord(host, VOYAGER_CONFIG.ip, mode) }.await()
//                 log("Exited addDnsRecord method", LogType.TRACE)
//                 var cloudflareId = ""
//
//
//                 cloudflareResult
//                     .onLeft { left: Array<CloudflareError> ->
//                         log("Cloudflare returned errors, trying to get the DNS record from database..", LogType.WARN)
//                         val found = findByHost(host)
//                         if (found == null) {
//                             log("DNS record was not found, aborting..", LogType.ERROR)
//                             return@withContext Either.Left(left
//                                 .foldIndexed("") {
//                                     index: Int,
//                                     acc: String,
//                                     crr: CloudflareError ->
//                                     (
//                                         "$acc ${crr.message}${if (index != left.size-1) {";"} else {""}}"
//                                     )
//                                 })
//                         }
//
//                         cloudflareId = found.dnsRecordId
//                     }
//                     .onRight { right: String -> cloudflareId = right }
//
//                 log("Fetched Cloudflare DNS record id: $cloudflareId", LogType.DEBUG)
//
//                 val internalPort = DockerManager.findInternalDockerPort(dockerFile)
//
//                 val labels = TraefikManager.genTraefikLabels(host.replace(".", ""), host, internalPort)
//
//                 log("Sending request to build docker image from docker file $dockerFile", LogType.DEBUG)
//                 val dockerImageResult = DockerManager.buildDockerImage(setOf(id), dockerFile, labels)
//                 var dockerImage = ""
//
//                 dockerImageResult
//                     .onFailure { exception: Throwable ->
//                         log("Docker build failed, removing DNS record from Cloudflare", LogType.ERROR)
//                         CloudflareManager.INSTANCE.removeDnsRecord(cloudflareId)
//                         // TODO: remove failed deployment directory
//                         return@withContext Either.Left(exception.message ?: "")
//                     }
//                     .onSuccess { img: String -> dockerImage = img }
//
//                 log("Docker build was successful and returned container image $dockerImage", LogType.DEBUG)
//
//                 val port = PortFinder.findFreePort()
//
//                 log("Sending docker create and start container request with image $dockerImage..", LogType.DEBUG)
//                 val containerIdResult = DockerManager.createAndStartContainer(
//                     "$host-$mode",AppDbContext
//                     port, internalPort, dockerImage)
//
//                 var containerId = ""
//
//                 containerIdResult
//                     .onFailure { exception: Throwable ->
//                         log("Docker container creation and startup for deployment $host-$mode failed, removing DNS record from Cloudflare..", LogType.ERROR)
//                         CloudflareManager.INSTANCE.removeDnsRecord(cloudflareId)
//                         // TODO: remove failed deployment directory
//                         return@withContext Either.Left(exception.message ?: "")
//                     }
//                     .onSuccess { idResult: String -> containerId = idResult }
//
//                 log("Docker container creation and startup for deployment $host-$mode was successful, container id is $containerId", LogType.DEBUG)
//
//                 val deployment =
//                     Deployment(
//                         id,
//                         containerId,
//                         port,
//                         cloudflareId,
//                         mode,
//                         host,
//                         DeploymentState.DEPLOYED,
//                         directory,
//                         repoUrl,
//                         branch
//                     )
//
//                 async { deployment.save() }.await()
//
//                 DiscordManager.sendDeploymentMessage(deployment)
//
//                 // TODO: notify user via email
//
//                 return@withContext Either.Right(deployment)
//             }
//         }
//
//         suspend fun exists(id: String): Boolean {
//             return findById(id) != null
//         }
//     }
