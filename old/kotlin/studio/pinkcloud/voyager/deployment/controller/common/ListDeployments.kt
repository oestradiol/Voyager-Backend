package studio.pinkcloud.voyager.deployment.controller.common

import io.ktor.http.*
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.view.ListDeploymentsResponse
import studio.pinkcloud.voyager.utils.logging.log

suspend fun listDeployments(repoUrl: String?, branch: String?): ListDeploymentsResponse {
    log("Attempting to list deployments")

    val deployments = Deployment.findAllFilterByRepoUrlAndBranch(repoUrl, branch)

    return ListDeploymentsResponse(
        HttpStatusCode.OK.value,
        "OK",
        listOf(),
        deployments
    )
}