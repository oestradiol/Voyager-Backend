package studio.pinkcloud.voyager.deployment.controller.common

import io.ktor.http.*
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.utils.VoyagerResponse
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

suspend fun stopDeployment(deploymentKey: String?): VoyagerResponse {
    log("Stopping deployment ${deploymentKey ?: "null"}", LogType.INFO)
    if (deploymentKey == null) {
        log("Deployment key is null", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "Deployment key not provided"
        )
    }

    log("Finding deployment for deployment key $deploymentKey", LogType.DEBUG)
    val deployment = Deployment.find(deploymentKey)

    if (deployment == null) {
        log("Deployment not found for deployment key $deploymentKey", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.NotFound.value,
            "Deployment with given key was not found"
        )
    }

    return deployment.stopAndDelete().fold(
        {_ ->
            log("Deployment stopped for deployment key $deploymentKey", LogType.INFO)
            VoyagerResponse(
                HttpStatusCode.OK.value,
                "Deployment stopped"
            )
        },
        {err ->
            log("Deployment failed to stop for deployment key $deploymentKey", LogType.WARN)
            VoyagerResponse(
                HttpStatusCode.InternalServerError.value,
                "Deployment was unable to be stopped: $err"
            )
        }
    )

}
