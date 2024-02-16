package studio.pinkcloud.voyager.deployment.controller.common

import io.ktor.http.*
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.view.GetDeploymentResponse
import studio.pinkcloud.voyager.utils.logging.log

suspend fun getDeployment(id: String?): GetDeploymentResponse {
    log("Attempting to get deployment with id: $id")

    if (id == null) {
        return GetDeploymentResponse(
            HttpStatusCode.BadRequest.value,
            "Bad Request",
            listOf("Deployment ID is null"),
            null
        )
    }

    val deployment = Deployment.findById(id)
        ?: return GetDeploymentResponse(
            HttpStatusCode.NotFound.value,
            "Not Found",
            listOf("Deployment with given id was not found"),
            null
        )

    return GetDeploymentResponse(
        HttpStatusCode.OK.value,
        "OK",
        listOf(),
        deployment
    )

}