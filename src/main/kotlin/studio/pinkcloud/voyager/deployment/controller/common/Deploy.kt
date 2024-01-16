package studio.pinkcloud.voyager.deployment.controller.common
import io.ktor.http.*
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.github.VoyagerGithub
import studio.pinkcloud.voyager.utils.VoyagerResponse
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.io.File

suspend fun deploy(
    deploymentKey: String?,
    repoUrl: String?,
    mode: DeploymentMode,
    subdomain: String?
): VoyagerResponse {
    log("Attempting to create deployment with deployment key: ${deploymentKey ?: "null"}, repository url: ${repoUrl ?: "null"}, mode: ${mode ?: "null"}, subdomain: ${subdomain ?: "null"}")


    if (deploymentKey == null) {
        log("Deployment key is null", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "No deployment key provided for deployment attempt with repository url $repoUrl and subdomain $subdomain"
        )
    }

    if (repoUrl == null) {
        log("Repository URL is null", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "No repository URL provided for deployment attempt $deploymentKey-$mode"
        )
    }

    if (!repoUrl.lowercase().startsWith("${VOYAGER_CONFIG.githubOrgName.lowercase()}/")) {
        log("Repository is not owned by PinkCloudStudios", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "Invalid repository URL $repoUrl for deployment attempt $deploymentKey, it is not owned by PinkCloudStudios"
        )
    }

    if (Deployment.exists(deploymentKey)) {
        log("Deployment already exists for given deployment key", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.Forbidden.value,
            "Deployment already exists for deployment attempt $deploymentKey"
        )
    }

    val projectDirectory: File = File("${VOYAGER_CONFIG.deploymentsDir}/$deploymentKey").also {
        log("Checking for old deployment directory", LogType.DEBUG)
        if (it.exists()) {
            log("Deleting old deployment directory..", LogType.DEBUG)
            it.deleteRecursively()
        }
    }

    log("Cloning from github repository..", LogType.DEBUG)
    try {
        Git
            .cloneRepository()
            .setURI("https://github.com/${repoUrl}")
            .setDirectory(projectDirectory)
            .setCredentialsProvider(VoyagerGithub.credentialsProvider)
            .call()
            .close()
    } catch (e: Exception) {
        log("Error cloning deployment from github repository: ${e.localizedMessage}", LogType.ERROR)
        return VoyagerResponse(
            HttpStatusCode.FailedDependency.value,
            "Failed to clone repository for deployment attempt $deploymentKey: ${e.message}"
        )
    }

    val dockerFile = File(projectDirectory, "Dockerfile")

    log("Checking for existence of Dockerfile in deployment directory", LogType.DEBUG)
    if (!dockerFile.exists()) {
        log("Dockerfile not found for deployment attempt $deploymentKey", LogType.WARN)
        return VoyagerResponse(
            HttpStatusCode.BadRequest.value,
            "Dockerfile for given repository was not found"
        )
    }
    log("Dockerfile found for $deploymentKey.", LogType.DEBUG)

    log("Formatting subdomain for $deploymentKey..", LogType.DEBUG)
    val subdomainNew = if (mode == DeploymentMode.PREVIEW) {
        "$deploymentKey-preview"
    } else {
        subdomain
    }

    log("Formatted subdomain for $deploymentKey is $subdomainNew", LogType.DEBUG)

    log("Calling Deployment.new function with args $deploymentKey, $dockerFile, $subdomainNew, $mode", LogType.DEBUG)
    val deploymentResult = Deployment.new(
        deploymentKey,
        dockerFile,
        subdomainNew,
        mode
    )

    return deploymentResult.fold<VoyagerResponse>(
        {left: String ->
            log("Deployment attempt $deploymentKey failed with errors: $left", LogType.WARN)
            return VoyagerResponse(
                HttpStatusCode.FailedDependency.value,
                "Deployment failed: $left"
            )},
        {right: Deployment ->
            log("Deployment attempt $deploymentKey was successful", LogType.INFO)
            return VoyagerResponse(
                HttpStatusCode.OK.value,
                "Deployed",
                right.dockerContainer
            )}
    )
}
