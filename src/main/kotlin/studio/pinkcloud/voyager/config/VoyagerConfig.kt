package studio.pinkcloud.voyager.config

import kotlinx.serialization.Serializable

@Serializable
data class VoyagerConfig(
    val cloudflareApiToken: String = "",
    val githubPat: String = "",
    val apiKey: String = "",
    val deploymentWebhook: String = "",
    val caddyFilePath: String = "/opt/pinkcloud/caddy/Caddyfile",
    val githubOrgName: String = "PinkCloudStudios",
    val deploymentsDir: String = "/opt/pinkcloud/voyager/deployments",
    val cloudflareZone: String = "",
    val supabaseUrl: String = "",
    val supabaseKey: String = "",
    val IP: String = "",
    val redisUrl: String = "localhost",
    val redisPort: Int = 6379,
    val forceRedisSync: Boolean = false,
    val isDevelopment: Boolean = true,
)
