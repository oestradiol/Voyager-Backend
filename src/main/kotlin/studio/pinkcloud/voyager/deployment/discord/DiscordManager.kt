package studio.pinkcloud.voyager.deployment.discord

import club.minnced.discord.webhook.WebhookClientBuilder
import club.minnced.discord.webhook.receive.ReadonlyMessage
import club.minnced.discord.webhook.send.WebhookEmbed
import club.minnced.discord.webhook.send.WebhookEmbedBuilder
import club.minnced.discord.webhook.send.WebhookMessageBuilder
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

object DiscordManager {
    private val webhookClient by lazy {
        WebhookClientBuilder(VOYAGER_CONFIG.deploymentWebhook).apply {
            setThreadFactory { job ->
                Thread(job, "Discord Webhook Thread").apply {
                    isDaemon = true
                }
            }

            setWait(false) // trying setting this to false
        }.build()
    }

    fun sendDeploymentMessage(deployment: Deployment) {
        log("Sending deployment discord message for deployment $deployment", LogType.INFO)
        val mode = deployment.mode.toString()
        val message =
            WebhookEmbedBuilder().apply {
                setTitle(WebhookEmbed.EmbedTitle("New $mode deployment", "https://${deployment.host}"))
                setDescription("A new $mode deployment has been created.")
                addField(WebhookEmbed.EmbedField(true, "ID", deployment.id))
                addField(WebhookEmbed.EmbedField(true, "Port", deployment.port.toString()))
                addField(WebhookEmbed.EmbedField(true, "Docker Container", deployment.containerId))
            }.build()

        log("Deployment discord message to be sent: $message")

        val webhookMessage =
            WebhookMessageBuilder()
                .addEmbeds(message)
                .setUsername("Voyager $mode deployment")
                .build()

        webhookClient.send(webhookMessage)
            .whenCompleteAsync { _: ReadonlyMessage?, err: Throwable? ->
                err?.let { log("Error sending discord webhook message: ${err.message}", LogType.ERROR) }
            }
    }
}
