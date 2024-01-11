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

class PreviewDeploymentSystem : AbstractDeploymentSystem("preview") {
    override val deploymentsFile = File("preview-deployments.json") // TODO: change this to redis in future for multi instance support.

    override fun getCaddyFileContent(deployment: Deployment): String {
        return """
            
            ${deployment.deploymentKey}-preview.pinkcloud.studio {
                reverse_proxy localhost:${deployment.port}
            }
        """.trimIndent()
    }
}
