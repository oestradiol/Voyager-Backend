package studio.pinkcloud.voyager.deployment.cloudflare

import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.HttpStatusCode
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.VOYAGER_JSON
import studio.pinkcloud.voyager.deployment.cloudflare.responses.*
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.util.Calendar
import java.text.SimpleDateFormat
import arrow.core.Either
import arrow.core.raise.either

object CloudflareManager {
    private val httpClient = HttpClient()

    suspend fun addDnsRecord(deploymentKey: String, ip: String, mode: DeploymentMode, domain: String): Either<Array<CloudflareError>, String> {
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
                    "comment": "Voyager $mode for $deploymentKey | Deployed at ${now}"
                    }
                    """.trimIndent()
            )
        }

        var id: String?
        var errors: Array<CloudflareError>?
        try {
            id = VOYAGER_JSON.decodeFromString<AddDnsRecordSuccess>(
                AddDnsRecordSuccess.serializer(),
                response.bodyAsText()
            ).result.id

            return Either.Right(id)
        } catch (err: Exception) {
            errors = VOYAGER_JSON.decodeFromString<AddDnsRecordFailure>(
                AddDnsRecordFailure.serializer(),
                response.bodyAsText()
            ).errors.toTypedArray()

            return Either.Left(errors)
        }

    }

    suspend fun removeDnsRecord(cloudflareId: String): Either<Array<CloudflareError>, Unit> {
        val response = httpClient.delete("https://api.cloudflare.com/client/v4/zones/${VOYAGER_CONFIG.cloudflareZone}/dns_records/${cloudflareId}") {
            headers["Content-Type"] = "application/json"
            headers["Authorization"] = VOYAGER_CONFIG.cloudflareApiToken
        }

        if (response.status == HttpStatusCode.OK) { return Either.Right(Unit) }

        val errors: Array<CloudflareError>

        errors = VOYAGER_JSON.decodeFromString<DeleteDnsRecordFailure>(
            DeleteDnsRecordFailure.serializer(),
            response.bodyAsText()
        ).errors.toTypedArray()


        return Either.Left(errors)
    }
}
