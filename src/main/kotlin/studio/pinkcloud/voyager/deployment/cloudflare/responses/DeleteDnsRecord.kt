package studio.pinkcloud.voyager.deployment.cloudflare.responses

import kotlinx.serialization.Serializable

@Serializable
data class DeleteDnsRecordData(
    val id: String,
)

@Serializable
data class DeleteDnsRecordSuccess(
    val result: DeleteDnsRecordData,
)

@Serializable
data class DeleteDnsRecordFailure(
    val errors: List<CloudflareError>,
    val messages: List<CloudflareMessage>,
    val success: Boolean,
)
