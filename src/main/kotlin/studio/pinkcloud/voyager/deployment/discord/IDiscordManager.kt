package studio.pinkcloud.voyager.deployment.discord

interface IDiscordManager {
    fun sendDeploymentMessage(deploymentKey: String, port: Int, dockerContainer: String)
    
    companion object {
        /**
         * The main instance of the [IDiscordManager] until I decide to do DI.
         */
        val INSTANCE: IDiscordManager = DiscordManagerImpl()
    }
}