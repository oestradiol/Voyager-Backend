package studio.pinkcloud.voyager.utils.logging

import java.io.File
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.text.SimpleDateFormat
import java.util.*
import java.util.concurrent.ConcurrentLinkedQueue

class LoggerFileWriter {
    companion object {
        private var isLoaded = false
        private lateinit var logFileFull: File
        private lateinit var logFileLatest: File
        private val logFileName: String = SimpleDateFormat(LoggerSettings.logFileNameFormat).format(Calendar.getInstance().time)

        internal class LogEntry(
            val message: String,
            val logType: CustomLogType,
            val date: String,
            val threadName: String
        )

        // Store the logs that come before the FileWriter is loaded
        private var unloadedLogQueue = ConcurrentLinkedQueue<LogEntry>()

        fun load() {
            if(isLoaded) {
                log("FileWriter is already loaded!", LogType.ERROR)
                return
            }

            log("Loading LoggerFileWriter..", LogType.INFO)

            //Make sure the path has the correct format
            if(!LoggerSettings.saveDirectoryPath.endsWith("/")) LoggerSettings.saveDirectoryPath += "/"

            logFileFull = File("${LoggerSettings.saveDirectoryPath}${logFileName}.log")
            logFileLatest = File("${LoggerSettings.saveDirectoryPath}latest.log")

            // Create the directory if it doesn't exist
            if(!directoryExists(LoggerSettings.saveDirectoryPath)) {
                log("Specified log directory (${LoggerSettings.saveDirectoryPath}) was not found, creating one..", LogType.WARN)
                val path = Paths.get(LoggerSettings.saveDirectoryPath)
                Files.createDirectories(path)
                log("Log directory created!", LogType.INFO)
            }

            if(logFileFull.exists()) logFileFull.delete()
            if(logFileLatest.exists()) logFileLatest.delete()

            logFileFull.createNewFile()
            logFileLatest.createNewFile()

            isLoaded = true

            //Write all logs that came before the FileWriter is loaded
            unloadedLogQueue.forEach { writeToFile(it.message, it.logType, it.date, it.threadName) }

            log("LoggerFileWriter loaded succesfully.", LogType.INFO)

        }

        fun writeToFile(message: String, type: CustomLogType, date: String, threadName: String) {
            if(!isLoaded) {
                unloadedLogQueue.add(LogEntry(message, type, date, threadName))
                return
            }

            logFileFull.appendText("$date [$threadName] [${type.name}] $message\n")
            logFileLatest.appendText("$date [$threadName] [${type.name}] $message\n")
        }
    }
}

fun directoryExists(directoryPath: String): Boolean {
    val path: Path = Paths.get(directoryPath)
    return Files.exists(path) && Files.isDirectory(path)
}
