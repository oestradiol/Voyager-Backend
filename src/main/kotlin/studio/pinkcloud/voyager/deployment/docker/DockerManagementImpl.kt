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

class DockerManagementImpl : IDockerManager {
    
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

    override fun buildDockerImage(deploymentKey: String, dockerfile: File) {
        dockerClient
            .buildImageCmd()
            .withDockerfile(dockerfile)
            .withTags(
                setOf(deploymentKey)
            )
            .exec(object : ResultCallback.Adapter<BuildResponseItem>() {
                override fun onNext(item: BuildResponseItem?) {
                }
            })
            .awaitCompletion() // block until the image is built
    }

    override fun createAndStartContainer(deploymentKey: String, port: Int, internalPort: Int, dockerImage: String): String {
        val id = dockerClient
            .createContainerCmd(dockerImage)
            .withName("voyager-preview-$deploymentKey") // todo: update this with prod switch
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
            .startContainerCmd(id)
            .exec()
        
        return id
    }

    override fun restartContainer(dockerContainer: String) {
        if (isContainerRunning(dockerContainer)) dockerClient.stopContainerCmd(dockerContainer).exec()

        dockerClient.startContainerCmd(dockerContainer)
    }

    override fun isContainerRunning(dockerContainer: String): Boolean {
        return dockerClient.inspectContainerCmd(dockerContainer).exec().state.running ?: false
    }

    override fun stopContainerAndDelete(dockerContainer: String) {
        stopContainer(dockerContainer)
        deleteContainer(dockerContainer)
    }

    override fun stopContainer(dockerContainer: String) {
        dockerClient.stopContainerCmd(dockerContainer).exec()
    }

    override fun deleteContainer(dockerContainer: String) {
        dockerClient.removeContainerCmd(dockerContainer).exec()
    }

    override fun getLogs(dockerContainer: String): String {
        val logContainerCmd = dockerClient.logContainerCmd(dockerContainer).withStdOut(true).withStdErr(true)
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

        var logsStr = "Size of logs: ${logs.size}\n"
        for (line in logs) {
            logsStr += line + "\n"
        }

        return logsStr
    }
}
