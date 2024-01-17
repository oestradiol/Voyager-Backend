package studio.pinkcloud.voyager.utils.logging

import java.io.File

import java.io.FileWriter
import java.io.FileOutputStream
import studio.pinkcloud.voyager.utils.logging.LoggerFileWriter.Companion.log
import studio.pinkcloud.voyager.utils.logging.LogType
import java.io.FileOutputStream
import studio.pinkcloud.voyager.utils.logging.LoggerFileWriter.Companion.log
import studio.pinkcloud.voyager.utils.logging.LogType
=======
>>>>>>> origin/main
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
        private lateinit var fileWriterFull: FileWriter
        private lateinit var fileWriterLatest: FileWriter
        private val logFileName: String = SimpleDateFormat(LoggerSettings.logFileNameFormat).format(Calendar.getInstance().time)

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

            fileWriterFull = FileWriter(logFileFull, Charsets.UTF_8, true)
            fileWriterLatest = FileWriter(logFileLatest, Charsets.UTF_8, true)

            log("LoggerFileWriter loaded successfully.", LogType.INFO)

            isLoaded = true
        }

        fun writeToFile(message: String, type: CustomLogType, date: String, threadName: String) {

            val logMessage = "$date [${type.name}] [$threadName] $message\n"

            fileWriterFull.write(logMessage)
            fileWriterLatest.write(logMessage)

        }

        fun flush() {
            fileWriterFull.flush()
            fileWriterLatest.flush()
        }

        fun close() {
            fileWriterFull.close()
            fileWriterLatest.close()
        }
    }
}

fun directoryExists(directoryPath: String): Boolean {
    val path: Path = Paths.get(directoryPath)
    return Files.exists(path) && Files.isDirectory(path)
}
