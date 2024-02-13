
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