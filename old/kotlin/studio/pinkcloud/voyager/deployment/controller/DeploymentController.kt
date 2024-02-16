package studio.pinkcloud.voyager.deployment.controller

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import studio.pinkcloud.voyager.deployment.controller.common.*
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.deployment.view.DeployResponse
import studio.pinkcloud.voyager.deployment.view.GetLogsResponse
import studio.pinkcloud.voyager.deployment.view.StopResponse
import studio.pinkcloud.voyager.routing.annotations.LoggedIn
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

fun Application.configureDeploymentApi() {
    
    routing {
        @LoggedIn
        post("/deployment") {
            log("Request received at route /deployments (POST)", LogType.INFO)
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

                log("Sending response $response", LogType.DEBUG)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request: ${err.localizedMessage}", LogType.ERROR)
                log(err, LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    DeployResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        listOf(err.localizedMessage),
                        null
                    )
                )
            }
        }
        
        @LoggedIn
        get("/deployment/{id}/logs") {
            val id = call.request.header("X-ID") ?: call.parameters["id"]

            try {
                log("Request received at route /deployment/$id/logs (GET)", LogType.INFO)

                val response = getLogs(id)

                log("Sending response $response", LogType.DEBUG)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request: ${err.localizedMessage}", LogType.ERROR)
                log(err, LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    GetLogsResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        listOf(err.localizedMessage),
                        null
                    )
                )
            }
        }

        @LoggedIn
        delete("/deployment/{id}") {
            val id = call.request.header("X-ID") ?: call.parameters["id"]

            try {
                log("Request received at route /deployment/$id (DELETE)", LogType.INFO)

                val response = stopDeployment(id)

                log("Sending response $response", LogType.DEBUG)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request: ${err.localizedMessage}", LogType.ERROR)
                log(err, LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    StopResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        listOf(err.localizedMessage)
                    )
                )
            }
        }

        @LoggedIn
        get("/deployment/{id}") {
            val id = call.request.header("X-ID") ?: call.parameters["id"]

            try {
                log("Request received at route /deployment/$id (GET)", LogType.INFO)

                val response = getDeployment(id)

                log("Sending response $response", LogType.DEBUG)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request: ${err.localizedMessage}", LogType.ERROR)
                log(err, LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    StopResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        listOf(err.localizedMessage)
                    )
                )
            }
        }

        @LoggedIn
        get("/deployment") {
            log("Request received at route /deployment (GET)", LogType.INFO)

            val repoUrl = call.request.header("X-Repo-URL") ?: call.request.queryParameters["repoUrl"]
            val branch = call.request.header("X-Branch") ?: call.request.queryParameters["branch"]

            try {
                val response = listDeployments(repoUrl, branch)

                log("Sending response $response", LogType.DEBUG)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request: ${err.localizedMessage}", LogType.ERROR)
                log(err, LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    StopResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Internal Server Error",
                        listOf(err.localizedMessage)
                    )
                )
            }
        }
    }
}

