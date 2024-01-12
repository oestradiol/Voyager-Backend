package studio.pinkcloud.voyager.deployment.caddy

import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import studio.pinkcloud.voyager.deployment.data.Deployment
import java.io.File

class CaddyManagerImpl : ICaddyManager {
    override fun updateCaddyFile(withOurApi: Boolean) {
        val staticCaddyFile = VOYAGER_CONFIG.staticCaddyFilePath
        val filePath = VOYAGER_CONFIG.caddyFilePath

        var newContent: String =
            if (withOurApi) {
                """
                voyager-api.pinkcloud.studio {
                    reverse_proxy localhost:8765
                }
                """.trimIndent()
            } else {
                ""
            }
        
        newContent += "\n# Static Configurations from $staticCaddyFile\n" + File(staticCaddyFile).readText()

        Deployment.findAll().forEach {
            newContent +=
                if (it.production) {
                    AbstractDeploymentSystem.PRODUCTION_INSTANCE.getCaddyFileContent(it)
                } else {
                    AbstractDeploymentSystem.PREVIEW_INSTANCE.getCaddyFileContent(it)
                }
        }

        File(filePath).writeText(newContent)
    }
}
