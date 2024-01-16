package studio.pinkcloud.voyager.deployment.model

import arrow.core.Either
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.caddy.CaddyManager
import studio.pinkcloud.voyager.deployment.cloudflare.CloudflareManager
import studio.pinkcloud.voyager.deployment.cloudflare.responses.CloudflareError
import studio.pinkcloud.voyager.deployment.discord.DiscordManager
import studio.pinkcloud.voyager.deployment.docker.DockerManager
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.utils.PortFinder
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.io.File

@Serializable
data class Deployment(
    val deploymentKey: String,
    val port: Int,
    val dockerContainer: String,
    val dnsRecordId: String,
    val mode: DeploymentMode,
    val domain: String, // full domain ex: test.pinkcloud.studio or pinkcloud.studio (can be either)
    var state: DeploymentState = DeploymentState.UNDEPLOYED,
    var createdAt: Long = System.currentTimeMillis(),
) {
    companion object {
        @OptIn(ExperimentalCoroutinesApi::class)
        val context = newSingleThreadContext("DeploymentThread")

        suspend fun find(deploymentKey: String): Deployment? {
            log("Finding deployment with deployment key $deploymentKey..", LogType.DEBUG)
            return withContext(context) {
                val found = redisClient.get("deployment:$deploymentKey")
                return@withContext found?.let { runCatching { Json.decodeFromString<Deployment>(found) }.getOrNull() }
            }
        }

        suspend fun findAll(): List<Deployment> {
            log("Finding all deployments..", LogType.DEBUG)
            val deploymentList = withContext(context) {
                log("Finding list of deployment keys from redis..", LogType.TRACE)
                val keys = redisClient.keys("deployment:*")?.toTypedArray()?.filterNotNull() ?: listOf()
                log("Keys found: ${keys.fold("") { acc: String, crr: String -> "$acc; $crr" }}", LogType.TRACE)
                if (keys.isEmpty()) return@withContext listOf()

                return@withContext redisClient
                    .mget(*(keys.toTypedArray()))
                    .filterNotNull()
                    .map { jsonStr: String -> runCatching { Json.decodeFromString<Deployment>(jsonStr) } }
                    .mapNotNull { result: Result<Deployment> -> result.getOrNull() }
            }

            log("Deployments found: ${deploymentList.fold("") { acc: String, crr: Deployment -> "$acc; $crr" }}", LogType.TRACE)

            return deploymentList
        }

        suspend fun new(
            deploymentKey: String,
            dockerFile: File,
            subdomain: String?,
            mode: DeploymentMode
        ): Either<String, Deployment> {
            return withContext(context) {
                log("Creating deployment with deployment key $deploymentKey, dockerFile: $dockerFile, subdomain: ${subdomain ?: "null"}, mode $mode", LogType.INFO)

                val domain = (subdomain ?: ".") + "pinkcloud.studio"

                log("Sending add DNS record request to cloudflare..", LogType.DEBUG)
                val cloudflareResult = CloudflareManager.addDnsRecord(deploymentKey, VOYAGER_CONFIG.IP, mode, domain)
                var cloudflareId = ""


                cloudflareResult
                    .onLeft { left: Array<CloudflareError> ->
                        log("Cloudflare returned errors, trying to get the DNS record from redis..", LogType.WARN)
                        val found = find(deploymentKey)
                        if (found == null) {
                            log("DNS record was not found, aborting..", LogType.ERROR)
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
                    }
                    .onRight { right: String -> cloudflareId = right }

                log("Fetched cloudflare DNS record id: $cloudflareId", LogType.DEBUG)

                log("Sending request to build docker image from docker file $dockerFile", LogType.DEBUG)
                val dockerImageResult = DockerManager.buildDockerImage(setOf(deploymentKey), dockerFile)
                var dockerImage = ""

                dockerImageResult
                    .onFailure { exception: Throwable ->
                        log("Docker build failed, removing DNS record from Cloudflare", LogType.ERROR)
                        CloudflareManager.removeDnsRecord(cloudflareId)
                        // TODO: remove failed deployment directory
                        return@withContext Either.Left(exception.message ?: "")
                    }
                    .onSuccess { img: String -> dockerImage = img }

                log("Docker build was successful and returned container image $dockerImage", LogType.DEBUG)

                val port = PortFinder.findFreePort()

                log("Sending docker create and start container request with name $deploymentKey-$mode..", LogType.DEBUG)
                val containerIdResult = DockerManager.createAndStartContainer(
                    "$deploymentKey-$mode",
                    port, DockerManager.findInternalDockerPort(dockerFile), dockerImage)

                var containerId = ""

                containerIdResult
                    .onFailure { exception: Throwable ->
                        log("Docker container creation and startup for deployment key $deploymentKey failed, removing DNS record from Cloudflare..", LogType.ERROR)
                        CloudflareManager.removeDnsRecord(cloudflareId)
                        // TODO: remove failed deployment directory
                        return@withContext Either.Left(exception.message ?: "")
                    }
                    .onSuccess { id: String -> containerId = id }

                log("Docker container creation and startup for deployment key $deploymentKey was successful, container id is $containerId", LogType.DEBUG)

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
            return withContext(context) {
                log("Checking if deployment for $deploymentKey exists on redis..", LogType.TRACE)
                redisClient.get("deployment:$deploymentKey") != null
            }
        }
    }

    suspend fun save() {
        val deployment = this
        withContext(context) {
            log("Saving deployment $deployment to redis..", LogType.DEBUG)
            redisClient.set("deployment:$deploymentKey", Json.encodeToString(serializer(), deployment))
        }
    }

    suspend fun deleteFromRedis(): Result<Unit> {
        return withContext(context) {
            try {
                log("Deleting deployment with key $deploymentKey from redis..", LogType.DEBUG)
                redisClient.del("deployment:$deploymentKey")

                return@withContext Result.success(Unit)
            } catch (err: Exception) {
                log("deployment deletion from redis for key $deploymentKey failed: ${err.localizedMessage}", LogType.ERROR)
                return@withContext Result.failure(err)
            }
        }
    }

    suspend fun delete(): Result<Unit> {
        val deployment = this
        return withContext(context) {
            // stop and remove docker container.
            log("Deleting deployment $deployment", LogType.INFO)
            if (state != DeploymentState.STOPPED) {
                log("Deployment is running", LogType.ERROR)
                return@withContext Result.failure(Exception("Tried to delete deployment that is not in stopped state: ${deployment}"))
            }
            DockerManager.deleteContainer(dockerContainer)

            // remove any existing files.
            File("${VOYAGER_CONFIG.deploymentsDir}/${deploymentKey}-$mode").also {
                log("Checking if directory for deployment with key $deploymentKey exists before deleting", LogType.DEBUG)
                if (it.exists()) {
                    log("It exists, deleting..", LogType.DEBUG)
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
            log("Stopping deployment $deployment", LogType.INFO)
            // stop docker container
            if (state != DeploymentState.DEPLOYED) {
                log("Deployment is not running", LogType.ERROR)
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
        return withContext(context) {
            log("Starting deployment with key $deploymentKey", LogType.INFO)
            if (state != DeploymentState.STOPPED) {
                log("Deployment with key $deploymentKey is not in stopped state", LogType.ERROR)
                return@withContext Result.failure(Exception("Tried to start deployment that is not in stopped state"))
            }

            log("Sending restart command to docker for container id $dockerContainer..", LogType.DEBUG)
            return@withContext DockerManager.restartContainer(dockerContainer).fold(
                {_ ->
                    log("Container restart was successful")
                    state = DeploymentState.DEPLOYED
                    save()
                    Result.success(Unit)
                },
                {err: Throwable ->
                    log("Container $dockerContainer restart failed with errors: ${err.localizedMessage}", LogType.ERROR)
                    Result.failure(err)
                }
            )
        }
    }

    suspend fun stopAndDelete(): Result<Unit> {
        log("Stopping and deleting deployment with key $deploymentKey", LogType.INFO)
        return stop().fold(
            {_ -> delete()},
            {err -> Result.failure(err)}
        )
    }

    suspend fun getLogs(): Result<String> {
        return withContext(context) {
            log("Getting logs for deployment $deploymentKey", LogType.INFO)
            DockerManager.getLogs(dockerContainer)
        }
    }

    suspend fun isRunning(): Result<Boolean> {
        return withContext(context) {
            log("Checking if deployment with key $deploymentKey is running..", LogType.DEBUG)
            if (state != DeploymentState.DEPLOYED) return@withContext Result.success(false)
            return@withContext DockerManager.isContainerRunning(dockerContainer)
        }
    }

    suspend fun restart(): Result<Unit> {
        return withContext(context) {
            log("Restarting deployment with key $deploymentKey", LogType.INFO)
            return@withContext stop().fold(
                {_ -> start()},
                {err: Throwable -> Result.failure(err)}
            )
        }
    }

    fun getCaddyFileContent(): String {
        log("Getting caddy file content for deployment $this", LogType.DEBUG)
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
