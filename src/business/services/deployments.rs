//         @OptIn(ExperimentalCoroutinesApi::class, DelicateCoroutinesApi::class)
//         val context = newSingleThreadContext("DeploymentThread")



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
//                 log("Sending add DNS record request to cloudflare..", LogType.DEBUG)
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
//                 log("Fetched cloudflare DNS record id: $cloudflareId", LogType.DEBUG)
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

//     suspend fun delete(): Result<Unit> {
//         val deployment = this
//         return withContext(context) {
//             // stop and remove docker container.
//             log("Deleting deployment $deployment", LogType.INFO)
//             if (state != DeploymentState.STOPPED) {
//                 log("Deployment is running", LogType.ERROR)
//                 return@withContext Result.failure(Exception("Tried to delete deployment that is not in stopped state: $deployment"))
//             }
//             DockerManager.deleteContainer(containerId)
//
//             // remove any existing files.
//             File(directory).also {
//                 log("Checking if directory for deployment with id $id exists before deleting", LogType.DEBUG)
//                 if (it.exists()) {
//                     log("It exists, deleting..", LogType.DEBUG)
//                     it.deleteRecursively()
//                 }
//             }
//
//             deleteFromDatabase()
//
//             // remove from cloudflare dns.[done]
//             CloudflareManager.INSTANCE.removeDnsRecord(dnsRecordId)
//
//             return@withContext Result.success(Unit)
//
//             // TODO: notify user via email
//         }
//     }

//     suspend fun stop(): Result<Unit> {
//         val deployment = this
//         return withContext(context) {
//             log("Stopping deployment $deployment", LogType.INFO)
//             // stop docker container
//             if (state != DeploymentState.DEPLOYED) {
//                 log("Deployment is not running", LogType.ERROR)
//                 return@withContext Result.failure(Exception("Tried to stop deployment that is not in deployed state: $deployment"))
//             }
//             state = DeploymentState.STOPPING
//             DockerManager.stopContainer(containerId)
//             state = DeploymentState.STOPPED
//             save()
//
//             return@withContext Result.success(Unit)
//         }
//     }

//     suspend fun start(): Result<Unit> {
//         return withContext(context) {
//             log("Starting deployment with id $id", LogType.INFO)
//             if (state != DeploymentState.STOPPED) {
//                 log("Deployment with id $id is not in stopped state", LogType.ERROR)
//                 return@withContext Result.failure(Exception("Tried to start deployment that is not in stopped state"))
//             }
//
//             log("Sending restart command to docker for container id $containerId..", LogType.DEBUG)
//             return@withContext DockerManager.restartContainer(containerId).fold(
//                 {_ ->
//                     log("Container restart was successful")
//                     state = DeploymentState.DEPLOYED
//                     save()
//                     Result.success(Unit)
//                 },
//                 {err: Throwable ->
//                     log("Container $containerId restart failed with errors: ${err.localizedMessage}", LogType.ERROR)
//                     Result.failure(err)
//                 }
//             )
//         }
//     }



//     suspend fun stopAndDelete(): Result<Unit> {
//         log("Stopping and deleting deployment with id $id", LogType.INFO)
//         return stop().fold(
//             {_ -> delete()},
//             {err -> Result.failure(err)}
//         )
//     }



//     suspend fun getLogs(): Result<List<String>> {
//         return withContext(context) {
//             log("Getting logs for deployment with id $id", LogType.INFO)
//             DockerManager.getLogs(containerId)
//         }
//     }



//     suspend fun isRunning(): Result<Boolean> {
//         return withContext(context) {
//             log("Checking if deployment with id $id is running..", LogType.DEBUG)
//             if (state != DeploymentState.DEPLOYED) return@withContext Result.success(false)
//             return@withContext DockerManager.isContainerRunning(containerId)
//         }
//     }

//     suspend fun restart(): Result<Unit> {
//         return withContext(context) {
//             log("Restarting deployment with id $id", LogType.INFO)
//             return@withContext stop().fold(
//                 {_ -> start()},
//                 {err: Throwable -> Result.failure(err)}
//             )
//         }
//     }