
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