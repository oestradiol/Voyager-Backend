package studio.pinkcloud.voyager.github

import org.eclipse.jgit.transport.UsernamePasswordCredentialsProvider
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.utils.Env

object VoyagerGithub {
    val credentialsProvider by lazy { 
        UsernamePasswordCredentialsProvider("PinkCloudStudios", VOYAGER_CONFIG.githubPat)
    } 
}