package studio.pinkcloud.voyager.deployment.health

import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import studio.pinkcloud.voyager.deployment.data.DeploymentState
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.*
import kotlin.system.*

class DockerHealthThread() : Thread() {
    override fun run() {
        while (true) {
            val tickDurationMillis = tick()

            // ensures that tick() delays for no less than 200ms and sleeps for at least 95% of the time
            sleep(TimeUnit.SECONDS.toMillis(Math.max(200, tickDurationMillis * 95)))
        }
    }

    // perform health checks to make sure that if any part of the deployment has gone wrong it either trys to
    // redeploy that part or just stops the deployment and cleans up & notifies the user.
    // returns elapsed synchronized block time in milliseconds
    private fun tick(): Long {
        // TODO: Update this for multi instances of [AbstractDeploymentSystem]
        val deployments = AbstractDeploymentSystem.deployments
        var elapsedTimeMillis: Long = 0
        for (deployment in deployments) {
            elapsedTimeMillis += measureTimeMillis {
                synchronized(deployment) {
                    runBlocking {
                        if (deployment.state != DeploymentState.DEPLOYED) return@runBlocking
                        if (AbstractDeploymentSystem.PREVIEW_INSTANCE.isRunning(deployment)) return@runBlocking

                        AbstractDeploymentSystem.PREVIEW_INSTANCE.restart(deployment)

                        if (!AbstractDeploymentSystem.PREVIEW_INSTANCE.isRunning(deployment)) {
                            AbstractDeploymentSystem.PREVIEW_INSTANCE.stop(deployment)
                            println("Deployment ${deployment.dockerContainer} has stopped.")
                            // TODO: notify the user that the deployment stopped
                        }
                    }
                }
            }
        }
        return elapsedTimeMillis
    }
}
