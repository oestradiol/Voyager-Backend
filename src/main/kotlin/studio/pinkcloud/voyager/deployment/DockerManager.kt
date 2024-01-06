package studio.pinkcloud.voyager.deployment

import com.github.dockerjava.api.DockerClient
import com.github.dockerjava.core.DefaultDockerClientConfig
import com.github.dockerjava.core.DockerClientImpl
import com.github.dockerjava.httpclient5.ApacheDockerHttpClient
import com.github.dockerjava.transport.DockerHttpClient

object DockerManager {
    private val dockerConfig: DefaultDockerClientConfig by lazy { 
        DefaultDockerClientConfig
            .createDefaultConfigBuilder()
            .withDockerHost("npipe://./pipe/docker_engine")
            .build()
    }
    
    private val dockerHttpClient: DockerHttpClient by lazy {    
        ApacheDockerHttpClient.Builder()
            .dockerHost(dockerConfig.dockerHost)
            .sslConfig(dockerConfig.sslConfig)
            .build()
    }
    
    val dockerClient by lazy {
        DockerClientImpl.getInstance(
            dockerConfig,
            dockerHttpClient
        )
    }
}