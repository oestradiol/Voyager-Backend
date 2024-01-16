package studio.pinkcloud.voyager.deployment.cloudflare

import arrow.core.Either
import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.VOYAGER_JSON
import studio.pinkcloud.voyager.deployment.cloudflare.responses.AddDnsRecordFailure
import studio.pinkcloud.voyager.deployment.cloudflare.responses.AddDnsRecordSuccess
import studio.pinkcloud.voyager.deployment.cloudflare.responses.CloudflareError
import studio.pinkcloud.voyager.deployment.cloudflare.responses.DeleteDnsRecordFailure
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.text.SimpleDateFormat
import java.util.*

object CloudflareManager {
    private val httpClient = HttpClient()

    suspend fun addDnsRecord(deploymentKey: String, ip: String, mode: DeploymentMode, domain: String): Either<Array<CloudflareError>, String> {
        log("Adding DNS record to cloudflare for deployment key: $deploymentKey, ip: $ip, mode: $mode, domain: $domain", LogType.INFO)

        val response = httpClient.post("https://api.cloudflare.com/client/v4/zones/${VOYAGER_CONFIG.cloudflareZone}/dns_records") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = VOYAGER_CONFIG.cloudflareApiToken

            val now = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss:SSSXXX").format(Calendar.getInstance().time)

            // we need to be careful when testing not to override existing dns undtil this is finished
            this.setBody(
                """
                    {
                    "content": "$ip",
                    "name": "${domain.split(".")[0].ifEmpty { "@" }}",
                    "proxied": true,
                    "type": "A",
                    "ttl": 1,
                    "comment": "Voyager $mode for $deploymentKey | Deployed at $now"
                    }
                    """.trimIndent()
            )
        }

        log("Request sent to cloudflare", LogType.DEBUG)
        log("Response: ${response.bodyAsText()}", LogType.DEBUG)

        val id: String?
        val errors: Array<CloudflareError>?
        try {
            log("Attempting to parse success cloudflare response..", LogType.DEBUG)

            id = VOYAGER_JSON.decodeFromString(
                AddDnsRecordSuccess.serializer(),
                response.bodyAsText()
            ).result.id

            log("Cloudflare request was successful with id: $id", LogType.DEBUG)

            return Either.Right(id)
        } catch (err: Exception) {
            log("Attempting to parse failure cloudflare response..", LogType.DEBUG)

            errors = VOYAGER_JSON.decodeFromString(
                AddDnsRecordFailure.serializer(),
                response.bodyAsText()
            ).errors.toTypedArray()

            log("Request failed with errors: $errors")

            return Either.Left(errors)
        }

    }

    suspend fun removeDnsRecord(dnsRecord: String): Either<Array<CloudflareError>, Unit> {
        log("Removing DNS record from cloudflare: $dnsRecord", LogType.INFO)

        val response = httpClient.delete("https://api.cloudflare.com/client/v4/zones/${VOYAGER_CONFIG.cloudflareZone}/dns_records/${dnsRecord}") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = VOYAGER_CONFIG.cloudflareApiToken
        }

        log("Request sent to cloudflare", LogType.DEBUG)
        log("Response: ${response.bodyAsText()}", LogType.DEBUG)

        if (response.status == HttpStatusCode.OK) {
            log("Cloudflare request was successful", LogType.DEBUG)
            return Either.Right(Unit)
        }

        log("Attempting to parse failure cloudflare response", LogType.DEBUG)

        val errors: Array<CloudflareError> = VOYAGER_JSON.decodeFromString(
            DeleteDnsRecordFailure.serializer(),
            response.bodyAsText()
        ).errors.toTypedArray()

        log("Cloudflare request failed with errors: $errors")

        return Either.Left(errors)
    }
}
