package studio.pinkcloud.voyager.deployment.docker

import java.io.File

interface IDockerManager {
    /**
     * Builds a docker image with the given [deploymentKey] and [dockerfile].
     *
     * @param deploymentKey The deployment key of the deployment.
     * @param dockerfile The dockerfile to use for the deployment.
     */
    fun buildDockerImage(
        deploymentKey: String,
        dockerfile: File
    ): String

    /**
     * Creates and starts a container with the given [deploymentKey], [port] and [dockerImage].
     *
     * @param deploymentKey The deployment key of the deployment.
     * @param port The port of the deployment.
     * @param dockerImage The docker image to use for the deployment.
     * @return The docker container id.
     */
    fun createAndStartContainer(
        deploymentKey: String,
        port: Int,
        internalPort: Int,
        dockerImage: String,
        domain: String
    ): String

    fun restartContainer(dockerContainer: String)

    fun isContainerRunning(dockerContainer: String): Boolean

    /**
     * Stops and deletes the container with the given [dockerContainer].
     *
     * @param dockerContainer The docker container id.
     */
    fun stopContainerAndDelete(dockerContainer: String)

    fun stopContainer(dockerContainer: String)

    fun deleteContainer(dockerContainer: String)

    /**
     * Gets the logs of the container with the given [dockerContainer].
     *
     * @param dockerContainer The docker container id.
     * @return The logs of the container.
     */
    fun getLogs(dockerContainer: String): String

    companion object {
        /**
         * The main instance of the [IDockerManager] until I decide to do DI.
         */
        val INSTANCE: IDockerManager = DockerManagementImpl()
    }
}
