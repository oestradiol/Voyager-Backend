package studio.pinkcloud.voyager.deployment.model

enum class DeploymentMode {
    PREVIEW,
    PRODUCTION,
    ;

    override fun toString(): String {
        return when (this) {
            PREVIEW -> "preview"
            PRODUCTION -> "production"
        }
    }
}
