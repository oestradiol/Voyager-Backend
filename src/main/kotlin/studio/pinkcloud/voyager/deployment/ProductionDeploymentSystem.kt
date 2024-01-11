package studio.pinkcloud.voyager.deployment

import kotlinx.serialization.encodeToString
import studio.pinkcloud.voyager.VOYAGER_JSON
import studio.pinkcloud.voyager.deployment.caddy.ICaddyManager
import studio.pinkcloud.voyager.deployment.cloudflare.ICloudflareManager
import studio.pinkcloud.voyager.deployment.data.*
import studio.pinkcloud.voyager.deployment.discord.IDiscordManager
import studio.pinkcloud.voyager.deployment.docker.IDockerManager
import studio.pinkcloud.voyager.utils.PortFinder
import java.io.File
import kotlinx.coroutines.*

class ProductionDeploymentSystem : AbstractDeploymentSystem("prod") {
    override val deploymentsFile = File("production-deployments.json") // TODO: change this to redis in future for multi instance support.

    override suspend fun deploy(
        deploymentKey: String,
        dockerFile: File,
    ): String {
        val id = super.deploy(deploymentKey, dockerFile)
        
        // notify client via email (check youtrack for email template)
        
        return id
    }
    
    override suspend fun delete(deployment: Deployment) {
        super.delete(deployment)
        // TODO: send email to client since this is a production website being deleted here & notify discord bot.
    }

    override fun getCaddyFileContent(deployment: Deployment): String {
        // right now all production deployments are hosted under our domain until client panel is started!
        return """
            
            ${deployment.deploymentKey}.pinkcloud.studio {
                reverse_proxy localhost:${deployment.port}
            }
        """.trimIndent()
    }
}
