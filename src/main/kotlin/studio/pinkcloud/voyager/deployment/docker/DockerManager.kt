package studio.pinkcloud.voyager.deployment.docker

import com.github.dockerjava.api.DockerClient
import com.github.dockerjava.api.async.ResultCallback
import com.github.dockerjava.api.model.*
import com.github.dockerjava.core.DefaultDockerClientConfig
import com.github.dockerjava.core.DockerClientImpl
import com.github.dockerjava.httpclient5.ApacheDockerHttpClient
import com.github.dockerjava.transport.DockerHttpClient
import java.io.Closeable
import java.io.File
import java.util.concurrent.atomic.AtomicInteger
import java.util.concurrent.Executors
import studio.pinkcloud.voyager.utils.logging.log
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.async
import kotlinx.coroutines.Deferred
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.asCoroutineDispatcher
import kotlinx.coroutines.withContext
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.deployment.model.Deployment
import kotlin.getOrThrow

object DockerManager {

    @OptIn(ExperimentalStdlibApi::class)
    private val mainContext = newSingleThreadContext("DockerThreadMain")
    
    private val dockerConfig: DefaultDockerClientConfig by lazy {
        DefaultDockerClientConfig
            .createDefaultConfigBuilder()
            .withDockerHost("unix:///var/run/docker.sock")
            .build()
    }

    private val dockerHttpClient: DockerHttpClient by lazy {
        ApacheDockerHttpClient.Builder()
            .dockerHost(dockerConfig.dockerHost)
            .sslConfig(dockerConfig.sslConfig)
            .build()
    }

    private val dockerClient: DockerClient by lazy {
        DockerClientImpl.getInstance(
            dockerConfig,
            dockerHttpClient
        )
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    suspend fun buildDockerImage(tags: Set<String>, dockerfile: File): Result<String> = coroutineScope {
        val context = newSingleThreadContext("DockerBuildThread-${dockerfile.hashCode()}")

        var dockerImage: String

        try {

            dockerImage = async(context) {
                var dockerImageBuilding = ""

                dockerClient
                    .buildImageCmd()
                    .withDockerfile(dockerfile)
                    .withTags(tags)
                    .exec(object : ResultCallback.Adapter<BuildResponseItem>() {
                        override fun onNext(item: BuildResponseItem?) {
                            item?.imageId?.let { dockerImageBuilding = item.imageId }
                        }
                    })
                    .awaitCompletion() // block until the image is built

                return@async dockerImageBuilding
            }.await()

        } catch (err: Exception) {
            log("Error trying to build docker image:", LogType.ERROR)
            log(err, LogType.ERROR)
            context.close()

            return@coroutineScope Result.failure(err)
        } finally {
            context.close()
        }

        return@coroutineScope Result.success(dockerImage)
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    suspend fun createAndStartContainer(
        name: String,
        port: Int,
        internalPort: Int,
        dockerImage: String
    ): Result<String> = coroutineScope {
        val context = newSingleThreadContext("DockerThread-${dockerImage.hashCode()}")

        val id: String

        try {
            id = async(context) {
                val idIn =
                    dockerClient
                        .createContainerCmd(dockerImage)
                        .withName(name)
                        // expose these ports inside the container
                        .withExposedPorts(
                            ExposedPort.tcp(internalPort)
                        )
                        .withHostConfig(
                            HostConfig.newHostConfig()
                                .withPortBindings(
                                    // map the ${internalPort} port inside the container to the ${port} port on the host
                                    PortBinding(
                                        Ports.Binding.bindPort(port),
                                        ExposedPort.tcp(internalPort)
                                    )
                                )
                        )
                        .exec()
                        .id // the id of the container that was created. (this container is not running yet)

                dockerClient
                    .startContainerCmd(idIn)
                    .exec()

                return@async idIn
            }.await()

        } catch (err: Exception) {
            log("Error while creating and starting container:", LogType.ERROR)
            log(err)
            context.close()

            return@coroutineScope Result.failure(err)
        } finally {
            context.close()
        }

        return@coroutineScope Result.success(id)

    }

    fun findInternalDockerPort(dockerFile: File): Int {
        return dockerFile.readText().substringAfter("EXPOSE ").substringBefore("\n").toInt()
    }

    suspend fun restartContainer(dockerContainer: String): Result<Unit> {
        return withContext(mainContext) {
            try {
                if (isContainerRunning(dockerContainer).getOrThrow()) dockerClient.stopContainerCmd(dockerContainer).exec()

                dockerClient.startContainerCmd(dockerContainer).exec()

                return@withContext Result.success(Unit)
            } catch (err: Exception) {
                return@withContext Result.failure(err)
            }
        }
    }

    suspend fun isContainerRunning(dockerContainer: String): Result<Boolean> {
        return withContext(mainContext) {
            try {
                return@withContext Result.success(
                    dockerClient.inspectContainerCmd(dockerContainer).exec().state.running ?: false
                )

            } catch (err: Exception) {
                return@withContext Result.failure(err)
            }
        }
    }

    suspend fun stopContainerAndDelete(dockerContainer: String): Result<Unit> {
        return stopContainer(dockerContainer).fold(
            {_ -> deleteContainer(dockerContainer)},
            {err -> Result.failure(err)}
        )
    }

    suspend fun stopContainer(dockerContainer: String): Result<Unit> {
        return withContext(mainContext) {
            try {
                dockerClient.stopContainerCmd(dockerContainer).exec()

                return@withContext Result.success(Unit)
            } catch (err: Exception) {
                return@withContext Result.failure(err)
            }
        }
    }

    suspend fun deleteContainer(dockerContainer: String): Result<Unit> {
        return withContext(mainContext) {
            try {
                dockerClient.removeContainerCmd(dockerContainer).exec()

                return@withContext Result.success(Unit)
            } catch (err: Exception) {
                return@withContext Result.failure(err)
            }
        }
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    suspend fun getLogs(dockerContainer: String): Result<String> = coroutineScope {
        val context = newSingleThreadContext("DockerLogThread-${dockerContainer.hashCode()}")

        var logsStr: String

        try {

            logsStr = async(context) {
                val logContainerCmd =
                    dockerClient
                        .logContainerCmd(dockerContainer)
                        .withStdOut(true)
                        .withStdErr(true)

                val logs = ArrayList<String>()

                try {
                    logContainerCmd.exec(object : ResultCallback.Adapter<Frame>() {
                                            override fun onNext(obj: Frame) {
                                                logs.add(obj.toString())
                                            }
                                        }).awaitCompletion()

                } catch (error: InterruptedException) {
                    error.printStackTrace()
                }

                var logsStrIn = "Size of logs: ${logs.size}\n"
                for (line in logs) {
                    logsStrIn += line + "\n"
                }
                return@async logsStrIn
            }.await()

        } catch (err: Exception) {
            log("Error getting logs from container:", LogType.ERROR)
            log(err, LogType.ERROR)
            context.close()
            return@coroutineScope Result.failure(err)
        } finally {
            context.close()
        }

        return@coroutineScope Result.success(logsStr)
    }
}
