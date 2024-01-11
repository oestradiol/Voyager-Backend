package studio.pinkcloud.voyager.deployment.data

import kotlinx.serialization.Serializable
import redis.clients.jedis.search.IndexDefinition
import redis.clients.jedis.search.IndexOptions
import redis.clients.jedis.search.Schema
import redis.clients.jedis.search.schemafields.TextField
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.utils.logging.*
import studio.pinkcloud.voyager.VOYAGER_CONFIG

@Serializable
data class Deployment(
    val deploymentKey: String,
    val port: Int,
    val dockerContainer: String,
    val dnsRecordId: String,
    val production: Boolean,
    var state: DeploymentState = DeploymentState.UNDEPLOYED,
    val createdAt: Long = System.currentTimeMillis(),
)

fun defineDeploymentRedisSchema() {
    log("Defining deployment redis schema..", LogType.INFORMATION)
    val schema =
        Schema().addTextField("$.deploymentKey", 1.0)
            .addNumericField("$.port")
            .addTextField("$.dockerContainer", 1.0)
            .addTextField("$.dnsRecordId", 1.0)
            .addNumericField("$.production")
            .addNumericField("$.state")
            .addNumericField("$.createdAt")

    val indexRule =
        IndexDefinition(IndexDefinition.Type.JSON)
            .setPrefixes("deployment:")

    try {
        redisClient.ftCreate("deployment-index", IndexOptions.defaultOptions().setDefinition(indexRule), schema)
    } catch (err: Exception) {
        if (err.message.equals("Index already exists") && VOYAGER_CONFIG.forceRedisSync) {
            redisClient.ftDropIndex("deployment-index")
            redisClient.ftCreate("deployment-index", IndexOptions.defaultOptions().setDefinition(indexRule), schema)
        }
    }
}
