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