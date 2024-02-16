package studio.pinkcloud.voyager.deployment.controller.common
import io.ktor.http.*
import org.eclipse.jgit.api.Git
import org.eclipse.jgit.storage.file.FileRepositoryBuilder
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.deployment.view.DeployResponse
import studio.pinkcloud.voyager.github.VoyagerGithub
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.io.File

suspend fun deploy(
    repoUrlRaw: String?,
    mode: DeploymentMode,
    subdomain: String?,
): DeployResponse {
    log("Attempting to create deployment with repository url: ${repoUrlRaw ?: "null"}, mode: $mode, subdomain: ${subdomain ?: "null"}")

    if (repoUrlRaw == null) {
        log("Repository URL is null", LogType.WARN)
        return DeployResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            listOf("No repository URL provided"),
            null
        )
    }

    val repoSplit = repoUrlRaw.split("@")
    val repoUrl = repoSplit[0]
    val branch = runCatching { repoSplit[1] }.getOrNull()

    log("repoUrl is $repoUrl, branch is $branch for $repoUrlRaw", LogType.DEBUG)

    if (!repoUrl.lowercase().startsWith("${VOYAGER_CONFIG.githubOrgName.lowercase()}/")) {
        log("Repository is not owned by ${VOYAGER_CONFIG.githubOrgName}", LogType.WARN)
        return DeployResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            listOf("Invalid repository URL $repoUrlRaw, it is not owned by ${VOYAGER_CONFIG.githubOrgName}"),
            null
        )
    }

    log("Formatting host for $repoUrlRaw-$mode..", LogType.DEBUG)

    val host = ((subdomain ?: "") + if (mode == DeploymentMode.PREVIEW)
    { "-preview.pinkcloud.studio"} else { ".pinkcloud.studio" })
        .replace(Regex("^[.,-]+"), "")
        .replace(Regex("\\s"), "")

    log("Formatted host for $repoUrlRaw-$mode is $host", LogType.DEBUG)

    if (Deployment.findByHost(host) != null) {
        return DeployResponse(
            HttpStatusCode.Forbidden.value,
            "Failed",
            listOf("Deployment at host $host already exists."),
            null
        )
    }

    val dir = repoUrl.replace(VOYAGER_CONFIG.githubOrgName + "/", "") + "-${branch ?: "default"}"

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
            .setURI("https://github.com/$repoUrl")
            .setBranch(branch)
            .setDirectory(projectDirectory)
            .setCredentialsProvider(VoyagerGithub.credentialsProvider)
            .call()
            .close()
    } catch (e: Exception) {
        log("Error cloning deployment from github repository: ${e.localizedMessage}", LogType.ERROR)
        return DeployResponse(
            HttpStatusCode.FailedDependency.value,
            "Failed",
            listOf("Failed to clone repository $repoUrlRaw: ${e.message}"),
            null
        )
    }

    val repo = FileRepositoryBuilder()
        .setGitDir(File("$projectDirectory/.git"))
        .build()

    val actualBranch = repo.branch

    val dockerFile = File(projectDirectory, "Dockerfile")

    log("Checking for existence of Dockerfile in deployment directory", LogType.DEBUG)
    if (!dockerFile.exists()) {
        log("Dockerfile not found for $repoUrlRaw-$mode", LogType.WARN)
        return DeployResponse(
            HttpStatusCode.BadRequest.value,
            "Failed",
            listOf("Dockerfile for given repository was not found"),
            null
        )
    }
    log("Dockerfile found for $repoUrlRaw-$mode", LogType.DEBUG)

    log("Calling Deployment.new function with args $repoUrlRaw-$mode, $dockerFile, $host, $mode", LogType.DEBUG)
    val deploymentResult = Deployment.new(
        dockerFile,
        host,
        mode,
        projectDirectory.path,
        repoUrl,
        actualBranch
    )

    return deploymentResult.fold<DeployResponse>(
        {left: String ->
            log("Deployment attempt for $repoUrlRaw-$mode failed with errors: $left", LogType.WARN)
            return DeployResponse(
                HttpStatusCode.FailedDependency.value,
                "Failed",
                listOf("Deployment failed: $left"),
                null
            )},
        {right: Deployment ->
            log("Deployment attempt for $repoUrlRaw-$mode was successful", LogType.INFO)

            return DeployResponse(
                HttpStatusCode.OK.value,
                "Success",
                listOf(),
                right.id
            )
        }
    )
}
