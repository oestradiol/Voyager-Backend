package studio.pinkcloud.voyager.deployment.cloudflare.responses

import kotlinx.serialization.Serializable

@Serializable
data class CloudflareError(
    val code: Int,
    val message: String,
)

@Serializable
data class CloudflareMessage(
    val code: Int,
    val message: String,
)
