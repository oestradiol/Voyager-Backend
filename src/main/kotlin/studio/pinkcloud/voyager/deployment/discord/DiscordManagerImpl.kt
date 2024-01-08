package studio.pinkcloud.voyager.deployment.discord

import club.minnced.discord.webhook.WebhookClientBuilder
import club.minnced.discord.webhook.send.WebhookEmbed
import club.minnced.discord.webhook.send.WebhookEmbedBuilder
import club.minnced.discord.webhook.send.WebhookMessageBuilder
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.utils.Env

class DiscordManagerImpl : IDiscordManager {
    
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
    
    override fun sendDeploymentMessage(deploymentKey: String, port: Int, dockerContainer: String) {
        val message = WebhookEmbedBuilder().apply { 
            setTitle(WebhookEmbed.EmbedTitle("New Preview Deployment", "https://$deploymentKey-preview.pinkcloud.studio"))
            setDescription("A new preview deployment has been created.")
            addField(WebhookEmbed.EmbedField(true, "Deployment Key", deploymentKey))
            addField(WebhookEmbed.EmbedField(true, "Port", port.toString()))
            addField(WebhookEmbed.EmbedField(true, "Docker Container", dockerContainer))
        }.build()
        
        val webhookMessage = WebhookMessageBuilder()
            .addEmbeds(message)
            .setUsername("Voyager Preview Deployment")
            .build()
        
        webhookClient.send(webhookMessage)
    }
}