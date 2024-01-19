package studio.pinkcloud.voyager.deployment.controller

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import studio.pinkcloud.voyager.deployment.controller.common.deploy
import studio.pinkcloud.voyager.deployment.controller.common.getLogs
import studio.pinkcloud.voyager.deployment.controller.common.stopDeployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.deployment.view.DeployResponse
import studio.pinkcloud.voyager.deployment.view.GetLogsResponse
import studio.pinkcloud.voyager.deployment.view.StopResponse
import studio.pinkcloud.voyager.routing.annotations.LoggedIn
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

fun Application.configureDeploymentApi() {
    
    routing() {
        @LoggedIn
        post("/deployment/deploy") {
            log("Request received at route /deployments/deploy", LogType.INFO)
            try {
                // this is just temp till supabase is implemented and getting project info from there can be done
                val repoURL = call.request.header("X-Repo-URL") ?: call.request.queryParameters["repoUrl"]
                val subdomain = call.request.header("X-Subdomain") ?: call.request.queryParameters["subdomain"]
                val modeStr = call.request.header("X-Mode") ?: call.request.queryParameters["mode"]

                val mode = when (modeStr) {
                    "preview" -> DeploymentMode.PREVIEW
                    "production" -> DeploymentMode.PRODUCTION
                    else -> DeploymentMode.PREVIEW
                }

                val response = deploy(repoURL, mode, subdomain)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (e: Exception) {
                log("Error processing request at route /deployments/deploy: ${e.localizedMessage}", LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    DeployResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        arrayOf(e.localizedMessage),
                        null
                    )
                )
                return@post
            }
        }
        
        @LoggedIn
        post("/deployment/{id}/logs") {
            val id = call.parameters["id"] ?: call.request.queryParameters["id"]

            try {
                log("Request received at route /deployment/$id/logs", LogType.INFO)

                val response = getLogs(id)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request at route /deployment/$id/logs", LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    GetLogsResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        arrayOf(err.localizedMessage),
                        null
                    ))

            }
        }

        @LoggedIn
        post("/deployment/{id}/stop") {
            val id = call.parameters["id"] ?: call.request.queryParameters["id"]

            try {
                log("Request received at route /deployment/$id/stop", LogType.INFO)

                val response = stopDeployment(id)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request at route /deployment/$id/stop", LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    StopResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        arrayOf(err.localizedMessage)
                    ))
            }
        }
    }
}

