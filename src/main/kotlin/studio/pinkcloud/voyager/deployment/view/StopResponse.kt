package studio.pinkcloud.voyager.deployment.view

import kotlinx.serialization.Serializable

@Serializable
data class StopResponse(
    val code: Int,
    val message: String,
    val errors: Array<String>
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as StopResponse

        if (code != other.code) return false
        if (message != other.message) return false
        if (!errors.contentEquals(other.errors)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = code
        result = 31 * result + message.hashCode()
        result = 31 * result + errors.contentHashCode()
        return result
    }
}
