package studio.pinkcloud.voyager.deployment.controller

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import studio.pinkcloud.voyager.deployment.data.Deployment
import studio.pinkcloud.voyager.github.VoyagerGithub
import studio.pinkcloud.voyager.routing.annotations.LoggedIn
import studio.pinkcloud.voyager.utils.VoyagerResponse
import java.io.File

fun Application.configureProductionDeployment() {
    routing() {

        post("/api/deployments/production") {

            // this is just temp till supabase is implemented and getting project info from there can be done
            val deploymentKey = call.request.header("X-Deployment-Key")
            val repoURL = call.request.header("X-Repo-URL")

            if (deploymentKey == null) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    VoyagerResponse(
                        success = false,
                        message = "No X-Deployment-Key or X-Repo-URL provided"
                    )
                )
                return@post
            }

            // ensures it is part of the pinkcloud studio org on github
            if (!repoURL!!.lowercase().startsWith("pinkcloudstudios/")) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    VoyagerResponse(
                        success = false,
                        message = "Invalid repo URL"
                    )
                )
                return@post
            }

            // deployment already exists
            if (AbstractDeploymentSystem.PRODUCTION_INSTANCE.deploymentExists(deploymentKey)) {
                call.respond(
                    HttpStatusCode.Conflict,
                    VoyagerResponse(
                        success = false,
                        message = "Deployment already exists"
                    )
                )
                return@post
            }

            val projectDirectory: File = File("/opt/pinkcloud/voyager/deployments/$deploymentKey-prod").also {
                if (it.exists()) {
                    it.deleteRecursively()
                }
            }

            try {
                Git
                    .cloneRepository()
                    .setURI("https://github.com/${repoURL}")
                    .setDirectory(projectDirectory)
                    .setCredentialsProvider(VoyagerGithub.credentialsProvider)
                    .call()
                    .close()
            } catch (e: Exception) {
                e.printStackTrace()
                call.respond(
                    HttpStatusCode.BadRequest,
                    VoyagerResponse(
                        success = false,
                        message = "Failed to clone repository"
                    )
                )
                return@post
            }

            val dockerFile = File(projectDirectory, "Dockerfile")

            if (!dockerFile.exists()) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    VoyagerResponse(
                        success = false,
                        message = "Dockerfile not found"
                    )
                )
                return@post
            }

            val containerId = AbstractDeploymentSystem.PRODUCTION_INSTANCE.deploy(deploymentKey, dockerFile)

            call.respond(
                HttpStatusCode.OK,
                VoyagerResponse(
                    success = true,
                    message = "Deployment created",
                    data = containerId
                )
            )
        }

        get("/api/deployments/production/{production}/logs") {
            val previewId = call.parameters["production"]

            if (previewId == null) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    VoyagerResponse(
                        success = false,
                        message = "No preview id provided"
                    )
                )
                return@get
            }

            val deployment = getAndValidate(previewId, call) ?: return@get

            call.respond(
                HttpStatusCode.OK,
                VoyagerResponse(
                    success = true,
                    message = "Logs retrieved",
                    data = AbstractDeploymentSystem.PRODUCTION_INSTANCE.getLogs(deployment)
                )
            )
        }

        post("/api/deployments/production/{production}/stop") {
            val previewId = call.parameters["production"]

            if (previewId == null) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    VoyagerResponse(
                        success = false,
                        message = "No production id provided"
                    )
                )
                return@post
            }

            val deployment = getAndValidate(previewId, call) ?: return@post

            try {
                AbstractDeploymentSystem.PRODUCTION_INSTANCE.stopAndDelete(deployment)
                call.respond(
                    HttpStatusCode.OK,
                    VoyagerResponse(
                        success = true,
                        message = "Deployment stopped"
                    )
                )
            } catch (err: Exception) {
                call.respond(
                    HttpStatusCode.Forbidden,
                    VoyagerResponse(
                        success = false,
                        message = err.localizedMessage
                    )
                )
            }
        }
    }
}

private suspend fun getAndValidate(previewId: String, call: ApplicationCall): Deployment? {
    val deployment = AbstractDeploymentSystem.PRODUCTION_INSTANCE.get(previewId)

    if (deployment == null) {
        call.respond(
            HttpStatusCode.NotFound,
            VoyagerResponse(
                success = false,
                message = "Deployment not found"
            )
        )

        return null
    }

    return deployment
}