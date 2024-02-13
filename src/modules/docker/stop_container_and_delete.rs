//     suspend fun stopContainerAndDelete(containerId: String): Result<Unit> {
//         log("Stopping and deleting container with id $containerId", LogType.INFO)
//         return stopContainer(containerId).fold(
//             {_ -> deleteContainer(containerId)},
//             {err -> Result.failure(err)}
//         )
//     }