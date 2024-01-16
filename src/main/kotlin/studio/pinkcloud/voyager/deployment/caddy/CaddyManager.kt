package studio.pinkcloud.voyager.deployment.caddy

import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.utils.logging.*
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentState
import java.io.File

object CaddyManager {
    suspend fun updateCaddyFile() {
        log("Updating caddy file..")

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

        log("Getting caddy file content for each deployment", LogType.DEBUG)
        Deployment.findAll().forEach {
            log("Deployment: $it", LogType.TRACE)
            if (it.state != DeploymentState.DEPLOYED) return
            log("Deployment: $it is deployed, adding caddy file content", LogType.TRACE)
            newContent += it.getCaddyFileContent()
        }

        log("Done", LogType.DEBUG)

        File(filePath).writeText(newContent)

        log("Caddy file updated")
        log("New caddy file:", LogType.TRACE)
        log(newContent, LogType.TRACE)
    }
}
