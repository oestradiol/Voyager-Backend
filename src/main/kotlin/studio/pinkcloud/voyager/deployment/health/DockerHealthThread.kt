package studio.pinkcloud.voyager.deployment.health

import java.util.concurrent.TimeUnit

class DockerHealthThread : Thread() {
    
    override fun run() {
        while (true) {
            sleep(TimeUnit.SECONDS.toMillis(20))
            tick()
        }
    }
    
    // perform health checks to make sure that if any part of the deployment has gone wrong it either trys to
    // redeploy that part or just stops the deployment and cleans up & notifies the user.
    private fun tick() {
        
    }
}