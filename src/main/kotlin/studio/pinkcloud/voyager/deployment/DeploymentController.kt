package studio.pinkcloud.voyager.deployment

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.util.*
import studio.pinkcloud.voyager.github.VoyagerGithub

fun Application.configureDeployment() {
    routing {
        post("/api/deployments/preview") {
            
            // this is just temp till supabase is implemented and getting project info from there can be done
            val deploymentKey = call.request.header("X-Deployment-Key")
            
            if (deploymentKey == null) {
                // TODO: Switch to custom response obj with error code and message
                call.respondText("No deployment key provided", status = HttpStatusCode.BadRequest)
                return@post
            }
            
            // deployment already exists
            if (DeploymentManager.deployments.containsKey(deploymentKey)) {
                
                // TODO: Confirmation to overwrite existing deployment, respond with error code and require a confirm header to be passed in
                call.respondText("Deployment already exists", status = HttpStatusCode.Conflict)
                return@post
            }
            
            val repoURL = call.request.header("X-Repo-URL")
            
            VoyagerGithub.cloneRepo(repoURL!!)
        }
    }
}