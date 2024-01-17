package studio.pinkcloud.voyager.deployment.health

<<<<<<< HEAD
import studio.pinkcloud.voyager.deployment.model.DeploymentState
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.*
=======
import kotlinx.coroutines.runBlocking
import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import studio.pinkcloud.voyager.deployment.data.Deployment
import studio.pinkcloud.voyager.deployment.data.DeploymentState
>>>>>>> origin/main
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.util.concurrent.TimeUnit
import kotlin.system.measureTimeMillis

class DockerHealthThread() : Thread() {
    override fun run() {
        try {
            while (true) {
                val tickDurationMillis = runBlocking { tick() }

                // ensures that tick() delays for no less than 200ms and sleeps for at least 95% of the time
                // t_elapsed% = t_elapsed/(t_delay + t_elapsed)
                // X = t_elapsed/(t_delay + t_elapsed) => X*t_delay = (1-X)*t_elapsed =>
                // => t_delay = t_elapsed * (1-x) / X
                // X = 5% = 0.05 => t_delay = 19 * t_elapsed
                sleep(TimeUnit.SECONDS.toMillis(Math.max(200, tickDurationMillis * 19)))
            }
        } catch (err: Exception) {
            log("Exception thrown while running docker health checks, sleeping thread for 1 second..")
            log(err)
            sleep(TimeUnit.SECONDS.toMillis(1000))
        }
    }

    // perform health checks to make sure that if any part of the deployment has gone wrong it either trys to
    // redeploy that part or just stops the deployment and cleans up & notifies the user.
    // returns elapsed synchronized block time in milliseconds
    private suspend fun tick(): Long {
        val deployments = Deployment.findAll()
        var elapsedTimeMillis: Long = 0
        for (deployment in deployments) {
            elapsedTimeMillis += measureTimeMillis {
                if (deployment.state != DeploymentState.DEPLOYED) return@measureTimeMillis

                if (deployment.mode == DeploymentMode.PRODUCTION) {
                    if (deployment.isRunning().getOrDefault(false)) { return@measureTimeMillis }

                    deployment.restart()

                    if (!deployment.isRunning().getOrDefault(false)) {
                        deployment.stop()
                        log("Production Deployment ${deployment.deploymentKey} has stopped.", LogType.WARN)
                        // TODO: notify the user that the deployment stopped
                    }
                } else {
                    if (deployment.isRunning().getOrDefault(false)) { return@measureTimeMillis }

                    deployment.restart()

                    if (!deployment.isRunning().getOrDefault(false)) {
                        deployment.stop()
                        log("Preview Deployment ${deployment.deploymentKey} has stopped.", LogType.WARN)
                        // TODO: notify the user that the deployment stopped
                    }
                }
            }
        }
        return elapsedTimeMillis
    }
}
