package studio.pinkcloud.voyager.utils.logging

import java.text.SimpleDateFormat
import java.util.Calendar
import com.github.ajalt.mordant.rendering.TextColors.*
import com.github.ajalt.mordant.rendering.TextStyles.*

object LoggerSettings {
    var saveToFile = true
    var saveDirectoryPath = "./logs/"
    var loggerStyle = LoggerStyle.TEXT_ONLY_BOLD
    var logFileNameFormat = "yyyy-MM-dd-HH:mm:ss"
    var minDisplaySeverity = LogType.INFO.severity
}

enum class LoggerStyle(val cast: (type: CustomLogType, msg: String, date: String, threadName: String) -> String) {
    FULL({type: CustomLogType,
          msg: String,
          date: String,
          threadName: String ->
             (black on type.color)("$date [$threadName] [${type.name}] $msg")
         }),
    PREFIX({type: CustomLogType,
            msg: String,
            date: String,
            threadName: String->
               (black on type.color)("$date [$threadName] [${type.name}]") +
                   type.color(" $msg")
           }),
    SUFFIX({type: CustomLogType,
            msg: String,
            date: String,
            threadName: String ->
               type.color("$date [$threadName] [${type.name}]") +
                   (black on type.color)(" $msg")
           }),
    TEXT_ONLY({type: CustomLogType,
               msg: String,
               date: String,
               threadName: String ->
                  type.color("$date [$threadName] [${type.name}] $msg")
              }),
    TEXT_ONLY_BOLD({type: CustomLogType,
                    msg: String,
                    date: String,
                    threadName: String ->
                       type.color(
                           bold("$date [$threadName] [${type.name}]") +
                               " $msg")
                   }),
}

fun log(
    message: String,
    type: CustomLogType = LogType.INFO,
) {
    // ISO 8601 date format
    val date = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss:SSSXXX").format(Calendar.getInstance().time)
    val threadName = Thread.currentThread().name

    if (LoggerSettings.saveToFile) LoggerFileWriter.writeToFile(message, type, date, threadName)

    if (LoggerSettings.minDisplaySeverity > type.severity) {
        return
    }

    println(LoggerSettings.loggerStyle.cast(type, message, date, threadName))
}

fun log(
    exception: Exception,
    logType: CustomLogType = LogType.FATAL,
) {
    log("$exception", logType)
    exception.stackTrace.forEach { log("   $it", logType) }
}

fun log(
    exception: Throwable,
    logType: CustomLogType = LogType.FATAL,
) {
    log("$exception", logType)
    exception.stackTrace.forEach { log("   $it", logType) }
}

fun logAndThrow(
    exception: Exception,
    logType: CustomLogType = LogType.FATAL,
) {
    log(exception, logType)
    throw exception
}

fun logAndThrow(
    message: String,
    logType: CustomLogType = LogType.FATAL,
) {
    logAndThrow(Exception(message), logType)
}
