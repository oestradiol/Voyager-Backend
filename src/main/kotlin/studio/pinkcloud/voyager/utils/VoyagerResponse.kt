package studio.pinkcloud.voyager.utils

import kotlinx.serialization.Contextual
import kotlinx.serialization.Serializable

@Serializable
data class VoyagerResponse(
    val success: Boolean,
    val message: String,
    val data: @Contextual Any? = null
)