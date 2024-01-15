package studio.pinkcloud.voyager.deployment.caddy

import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentState
import java.io.File

object CaddyManager {
    suspend fun updateCaddyFile() {
        val staticCaddyFile = VOYAGER_CONFIG.staticCaddyFilePath
        val filePath = VOYAGER_CONFIG.caddyFilePath

        var newContent: String =
            """
            voyager-api.pinkcloud.studio {
                reverse_proxy localhost:8765
            }
            """.trimIndent()

        newContent += "\n# Static Configurations from $staticCaddyFile\n" + File(staticCaddyFile).readText()

        newContent += "\n# Deployment Configurations"

        Deployment.findAll().forEach {
            if (it.state != DeploymentState.DEPLOYED) return
            newContent += it.getCaddyFileContent()
        }

        File(filePath).writeText(newContent)
    }
}
