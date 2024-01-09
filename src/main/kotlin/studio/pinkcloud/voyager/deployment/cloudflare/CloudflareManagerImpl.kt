package studio.pinkcloud.voyager.deployment.cloudflare

import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.VOYAGER_JSON
import studio.pinkcloud.voyager.deployment.cloudflare.responses.CloudflareResponse
import studio.pinkcloud.voyager.deployment.cloudflare.responses.CreateDNSData
import studio.pinkcloud.voyager.utils.Env
import studio.pinkcloud.voyager.utils.TimeUtils

class CloudflareManagerImpl : ICloudflareManager {
    
    private val httpClient = HttpClient()
    
    override suspend fun addDnsRecord(deploymentKey: String, ip: String, production: Boolean): String { 
        val response = httpClient.post("https://api.cloudflare.com/client/v4/zones/3b8a859109d691942925b0eb9ceb059e/dns_records") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = VOYAGER_CONFIG.cloudflareApiToken

            // we need to be careful when testing not to override existing dns undtil this is finished
            this.setBody(
                """
                    {
                      "content": "$ip",
                      "name": "${deploymentKey}${if (production) "" else "-preview"}",
                      "proxied": true,
                      "type": "A",
                      "ttl": 1,
                      "comment": "Voyager ${if (production) "Production Deployment" else "Preview"} for $deploymentKey | Deployed at ${TimeUtils.now()}"
                    }
                    """.trimIndent()
            )
        }
        
        return VOYAGER_JSON.decodeFromString(
            CloudflareResponse.serializer(CreateDNSData.serializer()),
            response.bodyAsText()
        ).result.id
    }

    override suspend fun removeDnsRecord(cloudflareId: String) {
        val response = httpClient.delete("https://api.cloudflare.com/client/v4/zones/3b8a859109d691942925b0eb9ceb059e/dns_records/${cloudflareId}") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = VOYAGER_CONFIG.cloudflareApiToken
        }
        
        println(response.status)
        println(response.bodyAsText())
    }
}