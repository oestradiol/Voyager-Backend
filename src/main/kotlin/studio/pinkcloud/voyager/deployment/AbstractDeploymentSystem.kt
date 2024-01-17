package studio.pinkcloud.voyager.deployment

import kotlinx.coroutines.runBlocking
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.cloudflare.ICloudflareManager
import studio.pinkcloud.voyager.deployment.data.Deployment
import studio.pinkcloud.voyager.deployment.data.DeploymentState
import studio.pinkcloud.voyager.deployment.discord.IDiscordManager
import studio.pinkcloud.voyager.deployment.docker.IDockerManager
import studio.pinkcloud.voyager.github.VoyagerGithub
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.utils.PortFinder
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.io.File

abstract class AbstractDeploymentSystem(val prefix: String) {

    open fun load() {
    }
    
    open suspend fun deploy(
        deploymentKey: String,
        dockerFile: File,
        domain: String
    ): String {
        // call deployment functions in IDockerManager [Done]
        // notify discord bot.
        // add to caddy. [Done]
        // add to deployment list  [Done]
        // add to cloudflare dns. [Done]

        // make sure this is done before adding to caddy or else caddy will fail because of SSL certs.
        var cloudflareId = ICloudflareManager.INSTANCE.addDnsRecord(deploymentKey, VOYAGER_CONFIG.ip, prefix.contains("prod"), domain)

        // if record already exists, get from deployments
        cloudflareId = cloudflareId ?: Deployment.find(deploymentKey)!!.dnsRecordId

        // build and deploy to docker.
        val dockerImage = IDockerManager.INSTANCE.buildDockerImage(deploymentKey, dockerFile)

        val port = PortFinder.findFreePort() // the port that the reverse proxy needs to use.

        // TODO: check for failed deployment
        val containerId =
            IDockerManager.INSTANCE.createAndStartContainer(
                deploymentKey,
                port,
                findInternalDockerPort(dockerFile),
                dockerImage,
                domain
            )


        val deployment =
            Deployment(
                deploymentKey,
                port,
                containerId,
                cloudflareId,
                prefix.contains("prod"),
                domain,
                DeploymentState.DEPLOYED
            )

        deployment.save()

        // add to caddy.
        // disabled since we switched to traefik.
        //ICaddyManager.INSTANCE.updateCaddyFile()

        // notify discord bot.
        IDiscordManager.INSTANCE.sendDeploymentMessage(deploymentKey, port, containerId)

        return containerId
    }

    suspend fun stopAndDelete(deployment: Deployment) {
        stop(deployment)
        delete(deployment)
    }

    fun stop(deployment: Deployment) {
        synchronized(deployment) {
            // stop docker container
            if (deployment.state != DeploymentState.DEPLOYED) throw Exception("Tried to stop deployment that is not deployed state: ${deployment}")
            deployment.state = DeploymentState.STOPPING
            IDockerManager.INSTANCE.stopContainer(deployment.dockerContainer)
            deployment.state = DeploymentState.STOPPED
            deployment.save()
        }
    }

    open suspend fun delete(deployment: Deployment) {
        synchronized(deployment) {
            // stop and remove docker container.
            if (deployment.state != DeploymentState.STOPPED) throw Exception("Tried to stop deployment that is not in stopped state: ${deployment}")
            IDockerManager.INSTANCE.deleteContainer(deployment.dockerContainer)

            // remove any existing files.
            File("${VOYAGER_CONFIG.deploymentsDir}/${deployment.deploymentKey}-$prefix").also {
                if (it.exists()) {
                    it.deleteRecursively()
                }
            }

            // remove from deployment list [done]
            deployment.delete()

            // remove from caddy after it is removed from internals deployments list. [done]
            // disabled since we switched to traefik.
            //ICaddyManager.INSTANCE.updateCaddyFile()

            // remove from cloudflare dns.[done]
            runBlocking { ICloudflareManager.INSTANCE.removeDnsRecord(deployment.dnsRecordId) }
        }
    }

    fun getLogs(deployment: Deployment): String {
        synchronized(deployment) {
            return IDockerManager.INSTANCE.getLogs(deployment.dockerContainer)
        }
    }

    fun deploymentExists(deploymentKey: String): Boolean {
        return redisClient.get("deployment:$deploymentKey") != null
    }

    private fun findInternalDockerPort(dockerFile: File): Int {
        return dockerFile.readText().substringAfter("EXPOSE ").substringBefore("\n").toInt()
    }

    fun isRunning(deployment: Deployment): Boolean {
        synchronized(deployment) {
            if (deployment.state != DeploymentState.DEPLOYED) return false
            return IDockerManager.INSTANCE.isContainerRunning(deployment.dockerContainer)
        }
    }

    suspend fun restart(deployment: Deployment) {
        if (deployment.state == DeploymentState.DEPLOYED) stopAndDelete(deployment)
        if (deployment.state != DeploymentState.STOPPED) return
    }
    
    fun cloneFromGithub(
        githubUrl: String,
        projectDirectory: File,
    ) {
        Git
            .cloneRepository()
            .setURI("https://github.com/$githubUrl")
            .setDirectory(projectDirectory)
            .setCredentialsProvider(VoyagerGithub.credentialsProvider)
            .call()
            .close()
    }

    companion object {
        
        /**
         * The main instance's of the [AbstractDeploymentSystem] until I decide to do DI.
         */
        val PREVIEW_INSTANCE: AbstractDeploymentSystem = PreviewDeploymentSystem()
        val PRODUCTION_INSTANCE: AbstractDeploymentSystem = ProductionDeploymentSystem()
    }
}
