package studio.pinkcloud.voyager.deployment

import studio.pinkcloud.voyager.deployment.data.Deployment
import java.io.File

interface IDeploymentSystem {
    fun load()
    suspend fun deploy(deploymentKey: String, dockerFile: File)
    fun stop(deployment: Deployment)
    fun delete(deployment: Deployment)
    fun getLogs(deployment: Deployment): String
    fun getCaddyFileContent(): String
    fun deploymentExists(deploymentKey: String): Boolean
    
    companion object {
        /**
         * The main instance of the [IDeploymentSystem] until I decide to do DI.
         */
        val INSTANCE: IDeploymentSystem = DeploymentSystemImpl()
    }
}