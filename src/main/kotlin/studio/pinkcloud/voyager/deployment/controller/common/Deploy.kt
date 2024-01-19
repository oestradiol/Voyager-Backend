package studio.pinkcloud.voyager.deployment.controller.common
import io.ktor.http.*
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.deployment.view.DeployResponse
import studio.pinkcloud.voyager.github.VoyagerGithub
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.io.File

suspend fun deploy(
    repoUrl: String?,
    mode: DeploymentMode,
    subdomain: String?,
): DeployResponse {
    log("Attempting to create deployment with repository url: ${repoUrl ?: "null"}, mode: ${mode ?: "null"}, subdomain: ${subdomain ?: "null"}")


    if (repoUrl == null) {
        log("Repository URL is null", LogType.WARN)
        return DeployResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            arrayOf("No repository URL provided"),
            null
        )
    }

    if (!repoUrl.lowercase().startsWith("${VOYAGER_CONFIG.githubOrgName.lowercase()}/")) {
        log("Repository is not owned by ${VOYAGER_CONFIG.githubOrgName}", LogType.WARN)
        return DeployResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            arrayOf("Invalid repository URL $repoUrl, it is not owned by ${VOYAGER_CONFIG.githubOrgName}"),
            null
        )
    }

    val host = (subdomain ?: "") + if (mode == DeploymentMode.PREVIEW)
    { "-preview.pinkcloud.studio"} else { ".pinkcloud.studio" }
        .replace(Regex("^[.,-]+"), "")
        .replace(Regex("\\s"), "")

    if (Deployment.findByHost(host) != null) {
        return DeployResponse(
            HttpStatusCode.Forbidden.value,
            "Failed",
            arrayOf("Deployment at host $host already exists."),
            null
        )
    }

    val dir = repoUrl.replace(VOYAGER_CONFIG.githubOrgName + "/", "")

    val projectDirectory: File = File("${VOYAGER_CONFIG.deploymentsDir}/$dir").also {
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
        return DeployResponse(
            HttpStatusCode.FailedDependency.value,
            "Failed",
            arrayOf("Failed to clone repository $repoUrl: ${e.message}"),
            null
        )
    }

    val dockerFile = File(projectDirectory, "Dockerfile")

    log("Checking for existence of Dockerfile in deployment directory", LogType.DEBUG)
    if (!dockerFile.exists()) {
        log("Dockerfile not found for $repoUrl", LogType.WARN)
        return DeployResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            arrayOf("Dockerfile for given repository was not found"),
            null
        )
    }
    log("Dockerfile found for $repoUrl", LogType.DEBUG)

    log("Formatting host for $repoUrl..", LogType.DEBUG)



    log("Formatted host for $repoUrl is $host", LogType.DEBUG)

    log("Calling Deployment.new function with args $repoUrl, $dockerFile, $host, $mode", LogType.DEBUG)
    val deploymentResult = Deployment.new(
        dockerFile,
        host,
        mode,
        projectDirectory.path
    )

    return deploymentResult.fold<DeployResponse>(
        {left: String ->
            log("Deployment attempt for $repoUrl-$mode failed with errors: $left", LogType.WARN)
            return DeployResponse(
                HttpStatusCode.FailedDependency.value,
                "Failed",
                arrayOf("Deployment failed: $left"),
                null
            )},
        {right: Deployment ->
            log("Deployment attempt for $repoUrl-$mode was successful", LogType.INFO)

            return DeployResponse(
                HttpStatusCode.OK.value,
                "Success",
                arrayOf(),
                right.id
            )
        }
    )
}
