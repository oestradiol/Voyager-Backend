package studio.pinkcloud.voyager.config

import kotlinx.serialization.Serializable

@Serializable
data class VoyagerConfig(
    @ConfigProperty("CLOUDFLARE_API_TOKEN")
    val cloudflareApiToken: String = "",

    @ConfigProperty("GITHUB_PAT")
    val githubPat: String = "",

    @ConfigProperty("ADMIN_API_KEY")
    val apiKey: String = "",

    @ConfigProperty("DEPLOYMENT_WEBHOOK")
    val deploymentWebhook: String = "",

    @ConfigProperty("GITHUB_ORG_NAME")
    val githubOrgName: String = "pinkcloudstudios",

    @ConfigProperty("DEPLOYMENTS_DIR")
    val deploymentsDir: String = "/opt/pinkcloud/voyager/deployments",

    @ConfigProperty("CLOUDFLARE_ZONE")
    val cloudflareZone: String = "",

    @ConfigProperty("SUPABASE_URL")
    val supabaseUrl: String = "",

    @ConfigProperty("SUPABASE_KEY")
    val supabaseKey: String = "",

    @ConfigProperty("IP")
    val ip: String = "",

    @ConfigProperty("REDIS_URL")
    val redisUrl: String = "localhost",
    
    @ConfigProperty("REDIS_PORT")
    val redisPort: Int = 6379,
    
    @ConfigProperty("FORCE_REDIS_SYNC")
    val forceRedisSync: Boolean = false,
    
    @ConfigProperty("MIN_LOG_DISPLAY")
    val minLogDisplay: String = "INFO",
    
    @ConfigProperty("IS_DEVELOPMENT")
    val isDevelopment: Boolean = true,
)

annotation class ConfigProperty(val envName: String)