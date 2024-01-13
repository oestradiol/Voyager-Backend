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
import studio.pinkcloud.voyager.VOYAGER_CONFIG

class ProductionDeploymentSystem : AbstractDeploymentSystem("prod") {

    override suspend fun deploy(deploymentKey: String, dockerFile: File, domain: String): String {
        val id = super.deploy(deploymentKey, dockerFile, domain)
        
        // notify client via email (check youtrack for email template)
        
        return id
    }
    
    override suspend fun delete(deployment: Deployment) {
        super.delete(deployment)
        // TODO: send email to client since this is a production website being deleted here & notify discord bot.
    }
}
