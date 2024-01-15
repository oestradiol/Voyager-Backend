package studio.pinkcloud.voyager.deployment.controller.common
import studio.pinkcloud.voyager.utils.VoyagerResponse
import io.ktor.http.HttpStatusCode
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.*
import studio.pinkcloud.voyager.github.VoyagerGithub
import java.io.File
import org.eclipse.jgit.api.Git

suspend fun deploy(
    deploymentKey: String?,
    repoUrl: String?,
    mode: DeploymentMode,
    subdomain: String?
): VoyagerResponse {
    if (deploymentKey == null) {
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "No deployment key provided"
        )
    }

    if (repoUrl == null) {
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "No repository URL provided"
        )
    }

    if (!repoUrl.lowercase().startsWith("${VOYAGER_CONFIG.githubOrgName.lowercase()}/")) {
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "Invalid repository URL, it is not owned by PinkCloudStudios"
        )
    }

    if (Deployment.exists(deploymentKey)) {
        return VoyagerResponse(
            HttpStatusCode.Forbidden.value,
            "Deployment with given key already exists"
        )
    }


    val projectDirectory: File = File("${VOYAGER_CONFIG.deploymentsDir}/$deploymentKey-$mode").also {
        if (it.exists()) {
            it.deleteRecursively()
        }
    }

    try {
        Git
            .cloneRepository()
            .setURI("https://github.com/${repoUrl}")
            .setDirectory(projectDirectory)
            .setCredentialsProvider(VoyagerGithub.credentialsProvider)
            .call()
            .close()
    } catch (e: Exception) {
        e.printStackTrace()
        return VoyagerResponse(
            HttpStatusCode.FailedDependency.value,
            "Failed to clone repository: ${e.message}"
        )
    }

    val dockerFile = File(projectDirectory, "Dockerfile")

    if (!dockerFile.exists()) {
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "Dockerfile for given repository was not found"
        )
    }

    val subdomainNew = if (mode == DeploymentMode.PREVIEW) {
        "$deploymentKey-preview"
    } else {
        subdomain
    }

    val deploymentResult = Deployment.new(
        deploymentKey,
        dockerFile,
        subdomainNew,
        mode
    )

    return deploymentResult.fold<VoyagerResponse>(
        {left: String -> return VoyagerResponse(
                             HttpStatusCode.FailedDependency.value,
                             "Deployment failed: $left"
                         )},
        {right: Deployment -> return VoyagerResponse(
                                  HttpStatusCode.OK.value,
                                  "Deployed",
                                  right.dockerContainer
                              )}
    )
}
