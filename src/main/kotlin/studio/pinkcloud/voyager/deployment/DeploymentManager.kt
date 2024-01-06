package studio.pinkcloud.voyager.deployment

object DeploymentManager {
    // deploymentKey -> port
    val deployments: MutableMap<String, Int> = mutableMapOf()
}