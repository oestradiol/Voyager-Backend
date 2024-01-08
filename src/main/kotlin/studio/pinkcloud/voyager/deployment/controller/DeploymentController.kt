package studio.pinkcloud.voyager.deployment.controller

import com.github.dockerjava.api.async.ResultCallback
import com.github.dockerjava.api.model.*
import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.deployment.IDeploymentSystem
import studio.pinkcloud.voyager.github.VoyagerGithub
import studio.pinkcloud.voyager.routing.annotations.LoggedIn
import studio.pinkcloud.voyager.utils.VoyagerResponse
import java.io.File
import kotlin.random.Random

fun Application.configureDeployment() {
    
    routing() {
        @LoggedIn
        post("/api/deployments/preview") {
            
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
            if (IDeploymentSystem.INSTANCE.deploymentExists(deploymentKey)) {
                call.respond(
                    HttpStatusCode.Conflict,
                    VoyagerResponse(
                        success = false,
                        message = "Deployment already exists"
                    )
                )
                return@post
            }
            
            val projectDirectory: File = File("/opt/pinkcloud/voyager/deployments/$deploymentKey-preview").also { 
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
            
            val containerId = IDeploymentSystem.INSTANCE.deploy(deploymentKey, dockerFile)
            
            call.respond(
                HttpStatusCode.OK,
                VoyagerResponse(
                    success = true,
                    message = "Deployment created",
                    data = containerId
                )
            )
        }
        
        get("/api/deployments/preview/{preview}/logs") {
            val previewId = call.parameters["preview"]
            
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
            
            val deployment = IDeploymentSystem.INSTANCE.get(previewId)
            
            if (deployment == null) {
                call.respond(
                    HttpStatusCode.NotFound,
                    VoyagerResponse(
                        success = false,
                        message = "Deployment not found"
                    )
                )
                return@get
            }
            
            call.respond(
                HttpStatusCode.OK,
                VoyagerResponse(
                    success = true,
                    message = "Logs retrieved",
                    data = IDeploymentSystem.INSTANCE.getLogs(deployment)
                )
            )
        }
        
        post("/api/deployments/preview/{preview_id}/stop") {
            val previewId = call.parameters["preview_id"]
            
            if (previewId == null) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    VoyagerResponse(
                        success = false,
                        message = "No preview id provided"
                    )
                )
                return@post
            }
            
            val deployment = IDeploymentSystem.INSTANCE.get(previewId)
            
            if (deployment == null) {
                call.respond(
                    HttpStatusCode.NotFound,
                    VoyagerResponse(
                        success = false,
                        message = "Deployment not found"
                    )
                )
                return@post
            }
            
            IDeploymentSystem.INSTANCE.stop(deployment)
            
            call.respond(
                HttpStatusCode.OK,
                VoyagerResponse(
                    success = true,
                    message = "Deployment stopped"
                )
            )
        }
    }
}