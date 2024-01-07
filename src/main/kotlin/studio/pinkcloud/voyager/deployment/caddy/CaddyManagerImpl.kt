package studio.pinkcloud.voyager.deployment.caddy

import java.io.File

class CaddyManagerImpl : ICaddyManager {

    override fun updateCaddyFile(content: String, withOurApi: Boolean) {
        val filePath = "/opt/pinkcloud/caddy/Caddyfile"
        val existingContent = File(filePath).readText()

        val updatedContent: String = if (withOurApi) {
            """         
            voyager-api.pinkcloud.studio {
                reverse_proxy localhost:8765
            }
            """.trimIndent() + "\n" + content
        } else {
            content
        }

        File(filePath).writeText(existingContent + "\n" + updatedContent)
    }

}