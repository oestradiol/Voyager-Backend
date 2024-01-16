package studio.pinkcloud.voyager.deployment.controller.common

import io.ktor.http.*
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.utils.VoyagerResponse
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

suspend fun getLogs(deploymentKey: String?): VoyagerResponse {
    log("Getting logs for $deploymentKey", LogType.INFO)

    if (deploymentKey == null) {
        log("Deployment ket is null", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "Deployment key not provided"
        )
    }

    log("Finding deployment for deployment key $deploymentKey", LogType.DEBUG)
    val deployment = Deployment.find(deploymentKey)

    if (deployment == null) {
        log("Deployment was not found for deployment key $deploymentKey", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.NotFound.value,
            "Deployment with given key was not found"
        )
    }

    log("Getting logs for deployment key $deploymentKey", LogType.DEBUG)
    val logsResult = deployment.getLogs()


    return logsResult.fold(
        {value: String ->
            log("Log retrieval for $deploymentKey was successful", LogType.INFO)
            return VoyagerResponse(HttpStatusCode.OK.value, "Logs retrieved", value)},
        {err: Throwable ->
            log("Log retrieval for $deploymentKey failed", LogType.WARN)
            VoyagerResponse(
                HttpStatusCode.InternalServerError.value,
                "Could not retrieve logs: ${err.message}"
            )}
    )

}
