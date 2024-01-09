package studio.pinkcloud.voyager.deployment.caddy

import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import java.io.File

class CaddyManagerImpl : ICaddyManager {

    override fun updateCaddyFile(withOurApi: Boolean) {
        val filePath = "/opt/pinkcloud/caddy/Caddyfile"

        var newContent: String = if (withOurApi) {
            """      
            voyager-api.pinkcloud.studio {
                reverse_proxy localhost:8765
            }
            """.trimIndent()
        } else {
            ""
        }
        
        AbstractDeploymentSystem.deployments.forEach {
            newContent += if (it.production) {
                AbstractDeploymentSystem.PRODUCTION_INSTANCE.getCaddyFileContent(it)
            } else {
                AbstractDeploymentSystem.PREVIEW_INSTANCE.getCaddyFileContent(it)
            }
        }

        File(filePath).writeText(newContent)
    }

}