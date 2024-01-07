package studio.pinkcloud.voyager.github

import org.eclipse.jgit.transport.UsernamePasswordCredentialsProvider
import studio.pinkcloud.voyager.utils.Env

object VoyagerGithub {
    val credentialsProvider = UsernamePasswordCredentialsProvider("PinkCloudStudios", Env.GITHUB_PAT)
}