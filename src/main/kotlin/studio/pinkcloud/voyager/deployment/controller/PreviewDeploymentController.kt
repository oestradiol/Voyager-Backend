package studio.pinkcloud.voyager.deployment.controller

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
<<<<<<< HEAD
import studio.pinkcloud.voyager.deployment.controller.common.deploy
import studio.pinkcloud.voyager.deployment.controller.common.getLogs
import studio.pinkcloud.voyager.deployment.controller.common.stopDeployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
=======
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import studio.pinkcloud.voyager.deployment.data.Deployment
import studio.pinkcloud.voyager.github.VoyagerGithub
>>>>>>> origin/main
import studio.pinkcloud.voyager.routing.annotations.LoggedIn
import studio.pinkcloud.voyager.utils.VoyagerResponse
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
<<<<<<< HEAD
=======
import java.io.File
>>>>>>> origin/main

fun Application.configurePreviewDeployment() {
    
    routing() {
        @LoggedIn
        post("/api/deployments/preview") {
            log("Request received at route /api/deployments/preview", LogType.INFO)
            try {
                // this is just temp till supabase is implemented and getting project info from there can be done
                val deploymentKey = call.request.header("X-Deployment-Key") ?: call.request.queryParameters["deploymentKey"]
                val repoURL = call.request.header("X-Repo-URL") ?: call.request.queryParameters["repoUrl"]
                val response = deploy(deploymentKey, repoURL, DeploymentMode.PREVIEW, null)
                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (e: Exception) {
                log("Error processing request at route /api/deployments/preview: ${e.localizedMessage}", LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    VoyagerResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Error processing request"
                    )
                )
                return@post
            }
        }
        
        @LoggedIn
        get("/api/deployments/preview/{deploymentKey}/logs") {
            val deploymentKey = call.parameters["deploymentKey"] ?: call.request.queryParameters["deploymentKey"]

            try {
                log("Request received at route /api/deployments/preview/${deploymentKey ?: "null"}/logs", LogType.INFO)

                val response = getLogs(deploymentKey)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request at route /api/deployments/preview/${deploymentKey ?: "null"}/logs: ${err.localizedMessage}", LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    VoyagerResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Error processing request"
                    ))

            }
        }

        @LoggedIn
        post("/api/deployments/preview/{deploymentKey}/stop") {
            val deploymentKey = call.parameters["deploymentKey"] ?: call.request.queryParameters["deploymentKey"]

            try {
                log("Request received at route /api/deployments/preview/${deploymentKey ?: "null"}/stop", LogType.INFO)

                val response = stopDeployment(deploymentKey)

                call.respond(
                    HttpStatusCode.fromValue(response.code),
                    response
                )
            } catch (err: Exception) {
                log("Error processing request at route /api/deployments/preview/${deploymentKey ?: "null"}/stop: ${err.localizedMessage}", LogType.ERROR)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    VoyagerResponse(
                        HttpStatusCode.InternalServerError.value,
                        "Error processing request"
                    ))
            }
        }
    }
}

