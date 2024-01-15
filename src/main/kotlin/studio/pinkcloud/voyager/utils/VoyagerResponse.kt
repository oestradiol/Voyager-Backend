package studio.pinkcloud.voyager.utils

import kotlinx.serialization.Serializable

@Serializable
data class VoyagerResponse(
    val code: Int,
    val message: String,
    val data: String? = null,
)
