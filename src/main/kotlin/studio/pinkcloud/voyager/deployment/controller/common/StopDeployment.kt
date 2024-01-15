package studio.pinkcloud.voyager.deployment.controller.common

import studio.pinkcloud.voyager.utils.VoyagerResponse
import studio.pinkcloud.voyager.deployment.model.*
import io.ktor.http.HttpStatusCode
import arrow.core.flatMap

suspend fun stopDeployment(deploymentKey: String?): VoyagerResponse {
    if (deploymentKey == null) {
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "Deployment key not provided"
        )
    }

    val deployment = Deployment.find(deploymentKey)

    if (deployment == null) {
        return VoyagerResponse(
            HttpStatusCode.NotFound.value,
            "Deployment with given key was not found"
        )
    }

    deployment.stopAndDelete()

    return VoyagerResponse(
        HttpStatusCode.OK.value,
        "Deployment stopped"
    )
}
