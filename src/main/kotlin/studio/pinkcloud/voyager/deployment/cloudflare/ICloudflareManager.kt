package studio.pinkcloud.voyager.deployment.cloudflare

interface ICloudflareManager {
    suspend fun addDnsRecord(deploymentKey: String, ip: String): String
    suspend fun removeDnsRecord(deploymentKey: String)
    
    companion object {
        /**
         * The main instance of the [ICloudflareManager] until I decide to do DI.
         */
        val INSTANCE: ICloudflareManager = CloudflareManagerImpl()
    }
}