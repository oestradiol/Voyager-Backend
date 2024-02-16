package studio.pinkcloud.voyager.deployment.controller.common

import io.ktor.http.*
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.view.StopResponse
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

suspend fun stopDeployment(id: String?): StopResponse {
    log("Stopping deployment ${id ?: "null"}", LogType.INFO)
    if (id == null) {
        log("Deployment id is null", LogType.WARN)
        return StopResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            listOf("Deployment id not provided")
        )
    }

    log("Finding deployment for deployment id $id", LogType.DEBUG)
    val deployment = Deployment.findById(id)

    if (deployment == null) {
        log("Deployment not found for deployment id $id", LogType.WARN)
        return StopResponse(
            HttpStatusCode.NotFound.value,
            "Failed",
            listOf("Deployment with given id was not found")
        )
    }

    return deployment.stopAndDelete().fold(
        {_ ->
            log("Deployment stopped for deployment id $id", LogType.INFO)
            StopResponse(
                HttpStatusCode.OK.value,
                "Success",
                listOf()
            )
        },
        {err ->
            log("Deployment failed to stop for deployment id $id", LogType.WARN)
            StopResponse(
                HttpStatusCode.InternalServerError.value,
                "Internal Server Error",
                listOf(err.localizedMessage)
            )
        }
    )

}
