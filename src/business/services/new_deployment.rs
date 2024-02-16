use crate::{business::repositories::deployments::find_by_host, modules::docker::find_internal_port};
use crate::configs::environment::HOST_IP;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
  types::model::deployment::{Deployment, Mode},
  Error,
};
//
// pub async fn new_deployment(
//   dockerfile: &String,
//   host: &String,
//   mode: &DeploymentMode,
//   directory: &String,
//   repo_url: &String,
//   branch: &String,
// ) -> Result<Deployment, Vec<Error>> {
//   event!(Level::INFO, "Creating deployment with host {host}, dockerfile {dockerfile}, mode {mode}, directory {directory}, repo_url {repo_url}, branch {branch}");
//
//   let id = Uuid::new_v4().to_string();
//
//   let cloudflare_result: Result<String, Vec<Error>> =
//     add_dns_record(host, &*HOST_IP, mode).await.map_err(|errs| {
//       errs
//         .iter()
//         .map(|err| Error::from(format!("CloudflareError {}: {}", err.code, err.message)))
//         .collect()
//     });
//
//   let cloudflare_id;
//
//   if cloudflare_result.is_ok() {
//     cloudflare_id = cloudflare_result.unwrap();
//   } else {
//     return Err(cloudflare_result.unwrap_err());
//   }
//
//   let internal_port = find_internal_port();
//
//   ()
// }

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
