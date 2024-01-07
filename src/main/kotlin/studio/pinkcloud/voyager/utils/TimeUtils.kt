package studio.pinkcloud.voyager.utils

import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

object TimeUtils {
    private fun formatDateTime(dateTime: LocalDateTime): String {
        val formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss")
        return dateTime.format(formatter)
    }
    
    fun now(): String = formatDateTime(LocalDateTime.now())
}