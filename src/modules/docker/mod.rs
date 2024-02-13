mod client;
mod build_image;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static!(
  pub static ref DOCKER_RUNTIME: Runtime = Runtime::new().unwrap();
);

//     private val dockerConfig: DefaultDockerClientConfig by lazy {
//         DefaultDockerClientConfig
//             .createDefaultConfigBuilder()
//             .withDockerHost("unix:///var/run/docker.sock")
//             .build()
//     }
//
//     private val dockerHttpClient: DockerHttpClient by lazy {
//         ApacheDockerHttpClient.Builder()
//             .dockerHost(dockerConfig.dockerHost)
//             .sslConfig(dockerConfig.sslConfig)
//             .build()
//     }
//
//     private val dockerClient: DockerClient by lazy {
//         DockerClientImpl.getInstance(
//             dockerConfig,
//             dockerHttpClient
//         )
//     }
//
//
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
//
//     fun findInternalDockerPort(dockerFile: File): Int {
//         log("Finding internal docker port for docker file $dockerFile", LogType.DEBUG)
//         return dockerFile.readText().substringAfter("EXPOSE ").substringBefore("\n").toInt()
//     }
//
//     suspend fun restartContainer(containerId: String): Result<Unit> {
//         log("Restarting container with id: $containerId", LogType.INFO)
//         return withContext(mainContext) {
//             try {
//                 if (isContainerRunning(containerId).getOrThrow()) {
//                     log("Stopping container with container id $containerId", LogType.DEBUG)
//                     dockerClient.stopContainerCmd(containerId).exec()
//                     log("Container with id $containerId stopped", LogType.DEBUG)
//                 }
//
//                 log("Starting container with id $containerId", LogType.DEBUG)
//                 dockerClient.startContainerCmd(containerId).exec()
//                 log("Container with id $containerId started")
//
//                 return@withContext Result.success(Unit)
//             } catch (err: Exception) {
//                 log("Error restarting container with id $containerId: ${err.localizedMessage}", LogType.ERROR)
//                 return@withContext Result.failure(err)
//             }
//         }
//     }
//
//     suspend fun isContainerRunning(containerId: String): Result<Boolean> {
//         log("Checking if container with id $containerId is running", LogType.DEBUG)
//         return withContext(mainContext) {
//             try {
//                 log("Inspecting container with id $containerId", LogType.TRACE)
//                 return@withContext Result.success(
//                     dockerClient.inspectContainerCmd(containerId).exec().state.running ?: false
//                 )
//
//             } catch (err: Exception) {
//                 log("Error checking if container with id $containerId is running: ${err.localizedMessage}", LogType.ERROR)
//                 return@withContext Result.failure(err)
//             }
//         }
//     }
//
//     suspend fun stopContainerAndDelete(containerId: String): Result<Unit> {
//         log("Stopping and deleting container with id $containerId", LogType.INFO)
//         return stopContainer(containerId).fold(
//             {_ -> deleteContainer(containerId)},
//             {err -> Result.failure(err)}
//         )
//     }
//
//     suspend fun stopContainer(containerId: String): Result<Unit> {
//         log("Stopping container with id $containerId", LogType.INFO)
//         return withContext(mainContext) {
//             try {
//                 log("Sending stop command to container with id $containerId", LogType.DEBUG)
//                 dockerClient.stopContainerCmd(containerId).exec()
//
//                 return@withContext Result.success(Unit)
//             } catch (err: Exception) {
//                 log("Stop command to container with id $containerId failed: ${err.localizedMessage}", LogType.ERROR)
//                 return@withContext Result.failure(err)
//             }
//         }
//     }
//
//     suspend fun deleteContainer(containerId: String): Result<Unit> {
//         log("Deleting container with id $containerId", LogType.INFO)
//         return withContext(mainContext) {
//             try {
//                 log("Sending remove command to container with id $containerId", LogType.DEBUG)
//                 dockerClient.removeContainerCmd(containerId).exec()
//
//                 return@withContext Result.success(Unit)
//             } catch (err: Exception) {
//                 log("Remove command to container with id $containerId failed: ${err.localizedMessage}", LogType.ERROR)
//                 return@withContext Result.failure(err)
//             }
//         }
//     }
//
//     suspend fun getLogs(containerId: String): Result<List<String>> = coroutineScope {
//         log("Getting logs for container with id $containerId", LogType.INFO)
//         val context = newSingleThreadContext("DockerLogThread-${containerId.hashCode()}")
//
//         val logs: List<String>
//
//         try {
//
//             logs = async(context) {
//                 log("Building log command for container id $containerId", LogType.DEBUG)
//                 val logContainerCmd =
//                     dockerClient
//                         .logContainerCmd(containerId)
//                         .withStdOut(true)
//                         .withStdErr(true)
//
//                 val logsIn = ArrayList<String>()
//
//                 try {
//                     log("Executing log command for container id $containerId", LogType.DEBUG)
//                     logContainerCmd.exec(object : ResultCallback.Adapter<Frame>() {
//                                             override fun onNext(obj: Frame) {
//                                                 log("Current log frame object: $obj", LogType.TRACE)
//                                                 logsIn.add(obj.toString())
//                                             }
//                                         }).awaitCompletion()
//
//                     log("Done executing log command for container id $containerId", LogType.DEBUG)
//
//                 } catch (error: InterruptedException) {
//                     log("Failed retrieving logs for container with id $containerId: ${error.localizedMessage}", LogType.ERROR)
//                     error.printStackTrace()
//                 }
//
//                 return@async logsIn
//             }.await()
//
//         } catch (err: Exception) {
//             log("Error getting logs from container: ${err.localizedMessage}", LogType.ERROR)
//             context.close()
//             return@coroutineScope Result.failure(err)
//         } finally {
//             context.close()
//         }
//
//         return@coroutineScope Result.success(logs)
//     }
// }
