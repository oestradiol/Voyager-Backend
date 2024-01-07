package studio.pinkcloud.voyager.deployment.cloudflare

import io.ktor.client.*
import io.ktor.client.request.*
import studio.pinkcloud.voyager.utils.Env
import studio.pinkcloud.voyager.utils.TimeUtils

class CloudflareManagerImpl : ICloudflareManager {
    
    private val httpClient = HttpClient()
    
    override suspend fun addDnsRecord(deploymentKey: String, ip: String) { 
        httpClient.post("https://api.cloudflare.com/client/v4/zones/3b8a859109d691942925b0eb9ceb059e/dns_records") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = Env.CLOUDFLARE_TOKEN

            this.setBody(
                """
                    {
                      "content": "$ip",
                      "name": "${deploymentKey}-preview",
                      "proxied": true,
                      "type": "A",
                      "ttl": 1,
                      "comment": "Voyager Preview for $deploymentKey | Deployed at ${TimeUtils.now()}"
                    }
                    """.trimIndent()
            )
        }
    }

    override fun removeDnsRecord(deploymentKey: String) {
        TODO("Not yet implemented")
    }
}