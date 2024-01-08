package studio.pinkcloud.voyager.deployment.cloudflare.responses

import kotlinx.serialization.Serializable

@Serializable
data class CreateDNSData(
    val id: String // there is a lot more data but this is all we need for now.
)