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
            .withName("voyager-preview-$deploymentKey")
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

    override fun stopContainerAndDelete(dockerContainer: String) {
        dockerClient
            .stopContainerCmd(dockerContainer)
            .exec()
        
        dockerClient
            .removeContainerCmd(dockerContainer)
            .exec()
    }

    override fun getLogs(dockerContainer: String): String {
        return "This is not yet implemented."
    }
}