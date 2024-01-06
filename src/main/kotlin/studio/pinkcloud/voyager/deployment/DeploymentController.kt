package studio.pinkcloud.voyager.deployment

import com.github.dockerjava.api.async.ResultCallback
import com.github.dockerjava.api.model.BuildResponseItem
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.util.*
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.github.VoyagerGithub
import java.io.File

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
            val projectDirectory: File = File("C:\\Users\\Natha\\OneDrive\\Desktop\\deployments\\${deploymentKey}")
            
            Git
                .cloneRepository()
                .setURI("https://github.com/${repoURL}")
                .setDirectory(projectDirectory)
                .setCredentialsProvider(VoyagerGithub.credentialsProvider)
                .call()
                .close()
            
            val dockerFile = File(projectDirectory, "Dockerfile")
            
            if (!dockerFile.exists()) {
                call.respondText("No Dockerfile found", status = HttpStatusCode.BadRequest)
                return@post
            }
            
            DockerManager.dockerClient
                .buildImageCmd(dockerFile /* <-- this can either be the folder or exact file */)
                .exec(object : ResultCallback.Adapter<BuildResponseItem>() {
                    override fun onNext(item: BuildResponseItem?) {
                        println(item)
                    }
                })
            
            println("Built image")
        }
    }
}