package studio.pinkcloud.voyager.config

import kotlinx.serialization.Serializable

@Serializable
data class VoyagerConfig(
    val cloudflareApiToken: String = "",
    val githubPat: String = "",
    val apiKey: String = "",
    val deploymentWebhook: String = "",
)