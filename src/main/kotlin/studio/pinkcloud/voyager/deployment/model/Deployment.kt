package studio.pinkcloud.voyager.deployment.model

import kotlinx.serialization.*
import kotlinx.serialization.json.Json
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.withContext
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.deployment.cloudflare.CloudflareManager
import java.io.File
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.docker.DockerManager
import arrow.core.Either
import studio.pinkcloud.voyager.deployment.cloudflare.responses.*
import studio.pinkcloud.voyager.utils.PortFinder
import studio.pinkcloud.voyager.deployment.caddy.CaddyManager
import org.eclipse.jgit.api.Git
import studio.pinkcloud.voyager.github.VoyagerGithub
import studio.pinkcloud.voyager.deployment.discord.DiscordManager

@Serializable
data class Deployment(
    val deploymentKey: String,
    val port: Int,
    val dockerContainer: String,
    val dnsRecordId: String,
    val mode: DeploymentMode,
    val domain: String, // full doamin ex: test.pinkcloud.studio or pinkcloud.studio (can be either)
    var state: DeploymentState = DeploymentState.UNDEPLOYED,
    var createdAt: Long = System.currentTimeMillis(),
) {
    companion object {
        @OptIn(ExperimentalCoroutinesApi::class)
        val context = newSingleThreadContext("DeploymentThread")

        suspend fun find(deploymentKey: String): Deployment? {
            return withContext(context) {
                val found = redisClient.get("deployment:$deploymentKey")
                return@withContext found?.let { runCatching { Json.decodeFromString<Deployment>(found) }.getOrNull() }
            }
        }

        suspend fun findAll(): List<Deployment> {
            return withContext(context) {
                val keys = redisClient.keys("deployment:*")?.toTypedArray()?.filterNotNull() ?: listOf()
                if (keys.isEmpty()) return@withContext listOf()

                return@withContext redisClient
                    .mget( *(keys.toTypedArray()) )
                    .filterNotNull()
                    .map { jsonStr: String -> runCatching { Json.decodeFromString<Deployment>(jsonStr) } }
                    .map { result: Result<Deployment> -> result.getOrNull() }
                    .filterNotNull()
            }
        }

        suspend fun new(
            deploymentKey: String,
            dockerFile: File,
            subdomain: String?,
            mode: DeploymentMode
        ): Either<String, Deployment> {
            return withContext(context) {
                val domain = (subdomain ?: ".") + "pinkcloud.studio"
                var cloudflareResult = CloudflareManager.addDnsRecord(deploymentKey, VOYAGER_CONFIG.IP, mode, domain)
                var cloudflareId = ""


                cloudflareResult
                    .onLeft({left: Array<CloudflareError> ->
                        val found = find(deploymentKey)
                        if (found == null) {
                            return@withContext Either.Left(left
                                .foldIndexed("") {
                                    index: Int,
                                    acc: String,
                                    crr: CloudflareError ->
                                    (
                                        "$acc ${crr.message}${if (index != left.size-1) {";"} else {""}}"
                                    )
                                })
                        }

                        cloudflareId = found.dnsRecordId
                    })
                    .onRight({right: String -> cloudflareId = right})

                val dockerImageResult = DockerManager.buildDockerImage(setOf(deploymentKey), dockerFile)
                var dockerImage = ""

                dockerImageResult
                    .onFailure({exception: Throwable ->
                        CloudflareManager.removeDnsRecord(cloudflareId)
                        // TODO: remove failed deployment directory
                        return@withContext Either.Left(exception.message ?: "")
                    })
                    .onSuccess({img: String -> dockerImage = img})


                val port = PortFinder.findFreePort()

                val containerIdResult = DockerManager.createAndStartContainer(
                    "$deploymentKey-$mode",
                    port, DockerManager.findInternalDockerPort(dockerFile), dockerImage)

                var containerId = ""

                containerIdResult
                    .onFailure({exception: Throwable ->
                        CloudflareManager.removeDnsRecord(cloudflareId)
                        // TODO: remove failed deployment directory
                        return@withContext Either.Left(exception.message ?: "")
                    })
                    .onSuccess({id: String -> containerId = id})

                val deployment =
                    Deployment(
                        deploymentKey,
                        port,
                        containerId,
                        cloudflareId,
                        mode,
                        domain
                    )

                CaddyManager.updateCaddyFile()

                DiscordManager.sendDeploymentMessage(deployment)

                // TODO: notify user via email

                return@withContext Either.Right(deployment)
            }
        }

        suspend fun exists(deploymentKey: String): Boolean {
            return withContext(context) { redisClient.get("deployment:$deploymentKey") != null }
        }
    }

    suspend fun save() {
        val deployment = this
        withContext(context) {
            redisClient.set("deployment:$deploymentKey", Json.encodeToString(serializer(), deployment))
        }
    }

    suspend fun deleteFromRedis(): Result<Unit> {
        return withContext(context) {
            try {
                redisClient.del("deployment:$deploymentKey")

                return@withContext Result.success(Unit)
            } catch (err: Exception) {
                return@withContext Result.failure(err)
            }
        }
    }

    suspend fun delete(): Result<Unit> {
        val deployment = this
        return withContext(context) {
            // stop and remove docker container.
            if (state != DeploymentState.STOPPED) {
                return@withContext Result.failure(Exception("Tried to delete deployment that is not in stopped state: ${deployment}"))
            }
            DockerManager.deleteContainer(dockerContainer)

            // remove any existing files.
            File("${VOYAGER_CONFIG.deploymentsDir}/${deploymentKey}-$mode").also {
                if (it.exists()) {
                    it.deleteRecursively()
                }
            }

            deleteFromRedis()

            // remove from caddy after it is removed from internals deployments list. [done]
            CaddyManager.updateCaddyFile()

            // remove from cloudflare dns.[done]
            CloudflareManager.removeDnsRecord(dnsRecordId)

            return@withContext Result.success(Unit)

            // TODO: notify user via email
        }
    }

    suspend fun stop(): Result<Unit> {
        val deployment = this
        return withContext(context) {
            // stop docker container
            if (state != DeploymentState.DEPLOYED) {
                return@withContext Result.failure(Exception("Tried to stop deployment that is not in deployed state: ${deployment}"))
            }
            state = DeploymentState.STOPPING
            DockerManager.stopContainer(dockerContainer)
            state = DeploymentState.STOPPED
            save()

            return@withContext Result.success(Unit)
        }
    }

    suspend fun start(): Result<Unit> {
        val deployment = this
        return withContext(context) {
            if (state != DeploymentState.STOPPED) {
                return@withContext Result.failure(Exception("Tried to start deployment that is not in stopped state"))
            }

            return@withContext DockerManager.restartContainer(dockerContainer).fold(
                {_ ->
                    state = DeploymentState.DEPLOYED
                    save()
                    Result.success(Unit)
                },
                {err: Throwable -> Result.failure(err)}
            )
        }
    }

    suspend fun stopAndDelete(): Result<Unit> {
        return stop().fold(
            {_ -> delete()},
            {err -> Result.failure(err)}
        )
    }

    suspend fun getLogs(): Result<String> {
        return withContext(context) {
            DockerManager.getLogs(dockerContainer)
        }
    }

    suspend fun isRunning(): Result<Boolean> {
        return withContext(context) {
            if (state != DeploymentState.DEPLOYED) return@withContext Result.success(false)
            return@withContext DockerManager.isContainerRunning(dockerContainer)
        }
    }

    suspend fun restart(): Result<Unit> {
        return withContext(context) {
            return@withContext stop().fold(
                {_ -> start()},
                {err: Throwable -> Result.failure(err)}
            )
        }
    }

    fun getCaddyFileContent(): String {
        return """

        $domain {
            reverse_proxy localhost:${port}

            tls {
                    dns cloudflare "${VOYAGER_CONFIG.cloudflareApiToken.replace("Bearer ", "")}"
            }
        }
    """.trimIndent()
    }
}
