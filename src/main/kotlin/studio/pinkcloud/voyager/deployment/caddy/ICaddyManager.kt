package studio.pinkcloud.voyager.deployment.caddy

interface ICaddyManager {
    fun updateCaddyFile(content: String,  withOurApi: Boolean = true)
    
    companion object {
        /**
         * The main instance of the [ICaddyManager] until I decide to do DI.
         */
        val INSTANCE: ICaddyManager = CaddyManagerImpl()
    }
}