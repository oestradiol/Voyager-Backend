package studio.pinkcloud.voyager.deployment

import studio.pinkcloud.voyager.deployment.data.Deployment
import java.io.File

class ProductionDeploymentSystem : AbstractDeploymentSystem("prod") {

    override suspend fun deploy(deploymentKey: String, dockerFile: File, domain: String): String {
        val id = super.deploy(deploymentKey, dockerFile, domain)
        
        // notify client via email (check youtrack for email template)
        
        return id
    }
    
    override suspend fun delete(deployment: Deployment) {
        super.delete(deployment)
        // TODO: send email to client since this is a production website being deleted here & notify discord bot.
    }
}
