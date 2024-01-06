package studio.pinkcloud.voyager.github

import org.kohsuke.github.GitHub
import org.kohsuke.github.GitHubBuilder

object VoyagerGithub {
    
    private val github: GitHub by lazy { 
        GitHubBuilder()
            .withOAuthToken(""/* */, "PinkCloudStudios")
            .build()
    }

    /**
     * @param repoURL The URL of the repo to clone. Ex. PinkCloudStudios/voyager-backend
     */
    fun cloneRepo(repoURL: String) {
        github.getRepository(repoURL).getDirectoryContent("/").forEach { 
            println("${it.name}/${it.path}/${it.type}")
        }
    }
}