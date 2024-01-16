package studio.pinkcloud.voyager.utils.logging

import arrow.core.Either
import com.github.ajalt.mordant.rendering.TextColors.black
import com.github.ajalt.mordant.rendering.TextStyles.bold
import java.text.SimpleDateFormat
import java.util.*
import java.util.concurrent.ConcurrentLinkedQueue

object LoggerSettings {
    var saveDirectoryPath = "./logs/"
    var loggerStyle = LoggerStyle.TEXT_ONLY_BOLD
    var logFileNameFormat = "yyyy-MM-dd'T'HH:mm:ss:SSSXXX"
    var minDisplaySeverity = LogType.INFO.severity
}

enum class LoggerStyle(val cast: (type: CustomLogType, msg: String, date: String, threadName: String) -> String) {
    FULL({type: CustomLogType,
          msg: String,
          date: String,
          threadName: String ->
             (black on type.color)("$date [${type.name}] [$threadName] $msg")
         }),
    PREFIX({type: CustomLogType,
            msg: String,
            date: String,
            threadName: String->
               (black on type.color)("$date [${type.name}] [$threadName]") +
                   type.color(" $msg")
           }),
    SUFFIX({type: CustomLogType,
            msg: String,
            date: String,
            threadName: String ->
               type.color("$date [${type.name}] [$threadName]") +
                   (black on type.color)(" $msg")
           }),
    TEXT_ONLY({type: CustomLogType,
               msg: String,
               date: String,
               threadName: String ->
                  type.color("$date [${type.name}] [$threadName] $msg")
              }),
    TEXT_ONLY_BOLD({type: CustomLogType,
                    msg: String,
                    date: String,
                    threadName: String ->
                       type.color(
                           bold("$date [${type.name}] [$threadName]") +
                               " $msg")
                   }),
}

class LogEntry(
    val content: Either<Throwable, String>,
    val type: CustomLogType,
    val date: Date,
    val threadName: String
)

object Logger {
    val logQueue: Queue<LogEntry> = ConcurrentLinkedQueue()
    private lateinit var loggerThread: Thread

    fun load() {
        loggerThread = object : Thread("LoggerThread") {
            override fun run() {
                try {
                    while (true) {
                        Thread.sleep(100)

                        while (logQueue.isNotEmpty()) {
                            logInternal(logQueue.element())
                            logQueue.remove()
                        }

                        LoggerFileWriter.flush()
                    }
                } catch (err: InterruptedException) {
                    log("Interruption caught in LoggerThread, cleaning up..", LogType.DEBUG)

                    while (logQueue.isNotEmpty()) {
                        logInternal(logQueue.element())
                        logQueue.remove()
                    }

                    LoggerFileWriter.flush()
                    LoggerFileWriter.close()
                }
            }
        }

        LoggerFileWriter.load()

        loggerThread.start()
    }

    fun logInternal(entry: LogEntry) {
        // ISO 8601 date format
        val date = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss:SSSXXX").format(entry.date)
        val content = entry.content
        val type = entry.type
        val threadName = entry.threadName

        if (LoggerSettings.minDisplaySeverity > type.severity) {
            content
                .onLeft {
                    LoggerFileWriter.writeToFile(it.message ?: "Exception in thread $threadName", type, date, threadName)

                    for (line in it.stackTrace) {
                        LoggerFileWriter.writeToFile("\t$line", type, date, threadName)
                    }
                }
                .onRight {
                    LoggerFileWriter.writeToFile(it, type, date, threadName)
                }
        } else {
            content
                .onLeft {
                    val message = it.message ?: "Exception in thread $threadName"

                    LoggerFileWriter.writeToFile(message, type, date, threadName)
                    println(LoggerSettings.loggerStyle.cast(type, message, date, threadName))

                    for (line in it.stackTrace) {
                        LoggerFileWriter.writeToFile("\t$line", type, date, threadName)
                        println(LoggerSettings.loggerStyle.cast(type, "\t$line", date, threadName))
                    }
                }
                .onRight {
                    LoggerFileWriter.writeToFile(it, type, date, threadName)
                    println(LoggerSettings.loggerStyle.cast(type, it, date, threadName))
                }
        }
    }

    fun cleanup() {
        loggerThread.interrupt()
    }
}

fun log(message: String, type: CustomLogType = LogType.INFO) {
    val threadName = Thread.currentThread().name

    synchronized(Logger) {
        val date = Calendar.getInstance().time
        Logger.logQueue.add(LogEntry(Either.Right(message), type, date, threadName))
    }
}


fun log(exception: Throwable, type: CustomLogType = LogType.FATAL) {
    val threadName = Thread.currentThread().name

    synchronized(Logger) {
        val date = Calendar.getInstance().time
        Logger.logQueue.add(LogEntry(Either.Left(exception), type, date, threadName))
    }
}

fun logAndThrow(exception: Exception, logType: CustomLogType = LogType.FATAL) {
    log(exception, logType)
    throw exception
}

fun logAndThrow(message: String, logType: CustomLogType = LogType.FATAL) {
    logAndThrow(Exception(message), logType)
}
