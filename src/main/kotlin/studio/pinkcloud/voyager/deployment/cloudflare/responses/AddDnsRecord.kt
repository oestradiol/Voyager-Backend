package studio.pinkcloud.voyager.deployment.cloudflare.responses

import kotlinx.serialization.Serializable


// https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-create-dns-record

@Serializable
data class Meta(
    val auto_added: Boolean,
    val source: String
)

@Serializable
data class AddDnsRecordData(
    val content: String, // ipv4 address
    val name: String,
    val proxied: Boolean,
    val type: String,
    val comment: String,
    val id: String,
    val locked: Boolean,
    val meta: Meta,
    val modified_on: String,
    val proxiable: Boolean,
    val tags: Array<String>,
    val ttl: Long,
    val zone_id: String,
    val zone_name: String,
)

@Serializable
data class AddDnsRecordSuccess(
    val errors: List<CloudflareError>,
    val messages: List<CloudflareMessage>,
    val result: AddDnsRecordData
)

@Serializable
data class AddDnsRecordFailure(
    val errors: List<CloudflareError>,
    val messages: List<CloudflareMessage>,
)
