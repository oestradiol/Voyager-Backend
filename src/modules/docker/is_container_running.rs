
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