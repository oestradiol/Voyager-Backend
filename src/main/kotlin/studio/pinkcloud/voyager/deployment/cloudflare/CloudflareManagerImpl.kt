package studio.pinkcloud.voyager.deployment.cloudflare

import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.VOYAGER_JSON
import studio.pinkcloud.voyager.deployment.cloudflare.responses.CloudflareResponse
import studio.pinkcloud.voyager.deployment.cloudflare.responses.CreateDNSData
import studio.pinkcloud.voyager.utils.TimeUtils
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

class CloudflareManagerImpl : ICloudflareManager {
    
    private val httpClient = HttpClient()
    
    override suspend fun addDnsRecord(deploymentKey: String, ip: String, production: Boolean, domain: String): String? {
        val response = httpClient.post("https://api.cloudflare.com/client/v4/zones/${VOYAGER_CONFIG.cloudflareZone}/dns_records") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = VOYAGER_CONFIG.cloudflareApiToken

            // we need to be careful when testing not to override existing dns undtil this is finished
            this.setBody(
                """
                    {
                      "content": "$ip",
                      "name": "${domain.split(".")[0].ifEmpty { "@" }}",
                      "proxied": true,
                      "type": "A",
                      "ttl": 1,
                      "comment": "Voyager ${if (production) "Production Deployment" else "Preview"} for $deploymentKey | Deployed at ${TimeUtils.now()}"
                    }
                    """.trimIndent()
            )
        }

        return try {
            VOYAGER_JSON.decodeFromString(
                CloudflareResponse.serializer(CreateDNSData.serializer()),
                response.bodyAsText()
            ).result.id
        } catch (err: Exception) {
            null
        }
    }

    override suspend fun removeDnsRecord(cloudflareId: String) {
        val response = httpClient.delete("https://api.cloudflare.com/client/v4/zones/${VOYAGER_CONFIG.cloudflareZone}/dns_records/${cloudflareId}") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = VOYAGER_CONFIG.cloudflareApiToken
        }
        log(response.status.toString(), LogType.INFORMATION)
        log(response.bodyAsText(), LogType.INFORMATION)
    }
}
