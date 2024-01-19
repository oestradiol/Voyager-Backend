package studio.pinkcloud.voyager.deployment.model

import arrow.core.Either
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import redis.clients.jedis.search.Document
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.cloudflare.CloudflareManager
import studio.pinkcloud.voyager.deployment.cloudflare.responses.CloudflareError
import studio.pinkcloud.voyager.deployment.discord.DiscordManager
import studio.pinkcloud.voyager.deployment.docker.DockerManager
import studio.pinkcloud.voyager.deployment.traefik.TraefikManager
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.utils.PortFinder
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.io.File
import java.util.*

@Serializable
data class Deployment(
    val id: String,
    val containerId: String,
    val port: Int,
    val dnsRecordId: String,
    val mode: DeploymentMode,
    val host: String, // full domain ex: test.pinkcloud.studio or pinkcloud.studio (can be either)
    var state: DeploymentState = DeploymentState.UNDEPLOYED,
    val directory: String,
    var createdAt: Long = System.currentTimeMillis(),
) {
    companion object {
        @OptIn(ExperimentalCoroutinesApi::class)
        val context = newSingleThreadContext("DeploymentThread")

        suspend fun findById(id: String): Deployment? {
            log("Finding deployment with id $id..", LogType.DEBUG)
            return withContext(context) {
                try {
                    val found = redisClient
                        .ftSearch("deploymentIndex", "@id:\"$id\"")
                        .documents[0]
                        .properties
                        .iterator()
                        .next()
                        .value

                    log("Deployment found: $found", LogType.DEBUG)
                    return@withContext runCatching { Json.decodeFromString(serializer(), found as String) }.getOrNull()
                } catch (err: IndexOutOfBoundsException) {
                    return@withContext null
                }
            }
        }

        suspend fun findByHost(host: String): Deployment? {
            log("Finding deployment by host $host", LogType.DEBUG)
            return withContext(context) {
                try {
                    val found = redisClient
                        .ftSearch("deploymentIndex", "@host:\"$host\"")
                        .documents[0]
                        .properties
                        .iterator()
                        .next()
                        .value

                    log("Deployment found: $found", LogType.DEBUG)
                    return@withContext runCatching { Json.decodeFromString(serializer(), found as String) }.getOrNull()
                } catch (err: IndexOutOfBoundsException) {
                    return@withContext null
                }
            }
        }

        suspend fun findAll(): List<Deployment> {
            log("Finding all deployments..", LogType.DEBUG)
            val deploymentList = withContext(context) {
                log("Finding list of deployment keys from redis..", LogType.TRACE)
                val found = redisClient
                    .ftSearch("deploymentIndex", "*")
                    ?.documents?.mapNotNull {
                        crr: Document -> runCatching {
                            Json.decodeFromString<Deployment>(
                                crr.properties
                                    .iterator()
                                    .next()
                                    .value as String
                            )
                        }.getOrNull()
                    }

                return@withContext found ?: listOf()

            }

            log("Deployments found: ${deploymentList.fold("") { acc: String, crr: Deployment -> "$acc; $crr" }}", LogType.TRACE)

            return deploymentList
        }

        suspend fun new(
            dockerFile: File,
            host: String,
            mode: DeploymentMode,
            directory: String
        ): Either<String, Deployment> {
            return withContext(context) {
                log("Creating deployment with host $host, dockerFile: $dockerFile, host: $host, mode $mode", LogType.INFO)

                val id = UUID.randomUUID().toString()

                log("Sending add DNS record request to cloudflare..", LogType.DEBUG)
                val cloudflareResult = CloudflareManager.addDnsRecord(host, VOYAGER_CONFIG.ip, mode)
                var cloudflareId = ""


                cloudflareResult
                    .onLeft { left: Array<CloudflareError> ->
                        log("Cloudflare returned errors, trying to get the DNS record from redis..", LogType.WARN)
                        val found = findByHost(host)
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

                val internalPort = DockerManager.findInternalDockerPort(dockerFile)

                val labels = TraefikManager.genTraefikLabels(host.replace(".", ""), host, internalPort)

                log("Sending request to build docker image from docker file $dockerFile", LogType.DEBUG)
                val dockerImageResult = DockerManager.buildDockerImage(setOf(id), dockerFile, labels)
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

                log("Sending docker create and start container request with image $dockerImage..", LogType.DEBUG)
                val containerIdResult = DockerManager.createAndStartContainer(
                    "$host-$mode",
                    port, internalPort, dockerImage)

                var containerId = ""

                containerIdResult
                    .onFailure { exception: Throwable ->
                        log("Docker container creation and startup for deployment $host-$mode failed, removing DNS record from Cloudflare..", LogType.ERROR)
                        CloudflareManager.removeDnsRecord(cloudflareId)
                        // TODO: remove failed deployment directory
                        return@withContext Either.Left(exception.message ?: "")
                    }
                    .onSuccess { id: String -> containerId = id }

                log("Docker container creation and startup for deployment $host-$mode was successful, container id is $containerId", LogType.DEBUG)

                val deployment =
                    Deployment(
                        id,
                        containerId,
                        port,
                        cloudflareId,
                        mode,
                        host,
                        DeploymentState.DEPLOYED,
                        directory
                    )

                DiscordManager.sendDeploymentMessage(deployment)

                // TODO: notify user via email

                return@withContext Either.Right(deployment)
            }
        }

        suspend fun exists(id: String): Boolean {
            return findById(id) != null
        }
    }

    suspend fun save() {
        val deployment = this
        withContext(context) {
            log("Saving deployment $deployment to redis..", LogType.DEBUG)
            redisClient.jsonSet("deployment:\"$id\"", Json.encodeToString(serializer(), deployment))
        }
    }

    suspend fun deleteFromRedis(): Result<Unit> {
        return withContext(context) {
            try {
                log("Deleting deployment with id $id from redis..", LogType.DEBUG)
                redisClient.jsonDel("deployment:\"$id\"")

                return@withContext Result.success(Unit)
            } catch (err: Exception) {
                log("deployment deletion from redis for id $id failed: ${err.localizedMessage}", LogType.ERROR)
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
            DockerManager.deleteContainer(containerId)

            // remove any existing files.
            File(directory).also {
                log("Checking if directory for deployment with id $id exists before deleting", LogType.DEBUG)
                if (it.exists()) {
                    log("It exists, deleting..", LogType.DEBUG)
                    it.deleteRecursively()
                }
            }

            deleteFromRedis()

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
                return@withContext Result.failure(Exception("Tried to stop deployment that is not in deployed state: $deployment"))
            }
            state = DeploymentState.STOPPING
            DockerManager.stopContainer(containerId)
            state = DeploymentState.STOPPED
            save()

            return@withContext Result.success(Unit)
        }
    }

    suspend fun start(): Result<Unit> {
        return withContext(context) {
            log("Starting deployment with id $id", LogType.INFO)
            if (state != DeploymentState.STOPPED) {
                log("Deployment with id $id is not in stopped state", LogType.ERROR)
                return@withContext Result.failure(Exception("Tried to start deployment that is not in stopped state"))
            }

            log("Sending restart command to docker for container id $containerId..", LogType.DEBUG)
            return@withContext DockerManager.restartContainer(containerId).fold(
                {_ ->
                    log("Container restart was successful")
                    state = DeploymentState.DEPLOYED
                    save()
                    Result.success(Unit)
                },
                {err: Throwable ->
                    log("Container $containerId restart failed with errors: ${err.localizedMessage}", LogType.ERROR)
                    Result.failure(err)
                }
            )
        }
    }

    suspend fun stopAndDelete(): Result<Unit> {
        log("Stopping and deleting deployment with id $id", LogType.INFO)
        return stop().fold(
            {_ -> delete()},
            {err -> Result.failure(err)}
        )
    }

    suspend fun getLogs(): Result<Array<String>> {
        return withContext(context) {
            log("Getting logs for deployment with id $id", LogType.INFO)
            DockerManager.getLogs(containerId)
        }
    }

    suspend fun isRunning(): Result<Boolean> {
        return withContext(context) {
            log("Checking if deployment with id $id is running..", LogType.DEBUG)
            if (state != DeploymentState.DEPLOYED) return@withContext Result.success(false)
            return@withContext DockerManager.isContainerRunning(containerId)
        }
    }

    suspend fun restart(): Result<Unit> {
        return withContext(context) {
            log("Restarting deployment with id $id", LogType.INFO)
            return@withContext stop().fold(
                {_ -> start()},
                {err: Throwable -> Result.failure(err)}
            )
        }
    }

}
