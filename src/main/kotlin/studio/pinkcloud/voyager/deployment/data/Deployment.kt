package studio.pinkcloud.voyager.deployment.data

import kotlinx.serialization.Serializable

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
