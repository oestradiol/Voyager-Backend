package studio.pinkcloud.voyager.deployment.controller.common

import io.ktor.http.*
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.view.GetLogsResponse
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

suspend fun getLogs(id: String?): GetLogsResponse {
    log("Getting logs for $id", LogType.INFO)

    if (id == null) {
        log("id is null", LogType.WARN)
        return GetLogsResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            listOf("Deployment id not provided"),
            null
        )
    }

    log("Finding deployment for id $id", LogType.DEBUG)
    val deployment = Deployment.findById(id)

    if (deployment == null) {
        log("Deployment was not found for id $id", LogType.WARN)
        return GetLogsResponse(
            HttpStatusCode.NotFound.value,
            "Failed",
            listOf("Deployment with given id was not found"),
            null
        )
    }

    log("Getting logs for id $id", LogType.DEBUG)
    val logsResult = deployment.getLogs()


    return logsResult.fold(
        {value: List<String> ->
            log("Log retrieval for $id was successful", LogType.INFO)
            return GetLogsResponse(
                HttpStatusCode.OK.value,
                "Logs retrieved",
                listOf(),
                value
            ) },
        {err: Throwable ->
            log("Log retrieval for $id failed", LogType.WARN)
            GetLogsResponse(
                HttpStatusCode.InternalServerError.value,
                "Internal Server Error",
                listOf(err.localizedMessage),
                null
            )}
    )

}
