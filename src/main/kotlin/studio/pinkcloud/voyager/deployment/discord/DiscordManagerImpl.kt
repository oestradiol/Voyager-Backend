package studio.pinkcloud.voyager.deployment.discord

import club.minnced.discord.webhook.WebhookClientBuilder
import club.minnced.discord.webhook.send.WebhookEmbed
import club.minnced.discord.webhook.send.WebhookEmbedBuilder
import club.minnced.discord.webhook.send.WebhookMessageBuilder
import club.minnced.discord.webhook.receive.ReadonlyMessage
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.Deployment
    import studio.pinkcloud.voyager.utils.logging.*

object DiscordManager {
    private val webhookClient by lazy {
        WebhookClientBuilder(VOYAGER_CONFIG.deploymentWebhook).apply {
            setThreadFactory { job ->
                Thread(job, "Discord Webhook Thread").apply {
                    isDaemon = true
                }
            }

            setWait(true)
        }.build()
    }

    suspend fun sendDeploymentMessage(deployment: Deployment) {
            val mode = deployment.mode.toString()
            val message =
                WebhookEmbedBuilder().apply {
                    setTitle(WebhookEmbed.EmbedTitle("New $mode deployment", "https://${deployment.domain}"))
                    setDescription("A new $mode deployment has been created.")
                    addField(WebhookEmbed.EmbedField(true, "Deployment Key", deployment.deploymentKey))
                    addField(WebhookEmbed.EmbedField(true, "Port", deployment.port.toString()))
                    addField(WebhookEmbed.EmbedField(true, "Docker Container", deployment.dockerContainer))
                }.build()

            val webhookMessage =
                WebhookMessageBuilder()
                    .addEmbeds(message)
                    .setUsername("Voyager $mode deployment")
                    .build()

            webhookClient.send(webhookMessage)
                .whenCompleteAsync(
                    {_: ReadonlyMessage?, err: Throwable? ->
                        err?.let { log("Error sending discord webhook message: ${err.message}") }
                    })
        }
}
