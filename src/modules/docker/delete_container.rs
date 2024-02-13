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