package studio.pinkcloud.voyager.deployment.data

import kotlinx.serialization.Serializable

@Serializable
data class Deployment(
    val deploymentKey: String,
    val port: Int,
    val dockerContainer: String
)