package studio.pinkcloud.voyager.deployment.cloudflare.responses

import kotlinx.serialization.Serializable

@Serializable
data class CloudflareResponse<T>(
    val errors: List<String>,
    val messages: List<String>,
    val result: T
)