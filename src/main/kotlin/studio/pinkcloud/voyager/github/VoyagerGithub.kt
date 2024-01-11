package studio.pinkcloud.voyager.github

import org.eclipse.jgit.transport.UsernamePasswordCredentialsProvider
import studio.pinkcloud.voyager.VOYAGER_CONFIG

object VoyagerGithub {
    val credentialsProvider by lazy {
        UsernamePasswordCredentialsProvider(VOYAGER_CONFIG.githubOrgName, VOYAGER_CONFIG.githubPat)
    }
}
