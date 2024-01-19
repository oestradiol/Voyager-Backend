package studio.pinkcloud.voyager

import com.charleskorn.kaml.Yaml
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import studio.pinkcloud.voyager.config.VoyagerConfig
import studio.pinkcloud.voyager.deployment.controller.configureDeploymentApi
import studio.pinkcloud.voyager.redis.connectToRedis
import studio.pinkcloud.voyager.redis.defineRedisSchema
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.Logger
import studio.pinkcloud.voyager.utils.logging.LoggerSettings
import studio.pinkcloud.voyager.utils.logging.log
import java.io.File
import kotlin.reflect.full.memberProperties

val programStartTime = System.currentTimeMillis()

fun main() {
    embeddedServer(
        Netty,
        port = 8765,
        host = "0.0.0.0",
        module = Application::init
    ).start(true)
}

fun Application.init() {

    try {
        Logger.load()
    } catch (err: Exception) {
        println("[FATAL] Error initializing logger")
    }

    val globalExceptionHandler =
        Thread.UncaughtExceptionHandler { thread, err ->
            try {
                log("Uncaught exception in thread ${thread.name}:", LogType.FATAL)
                log(err)

                Logger.cleanup()
            } catch (err2: Exception) {
                err.printStackTrace()
                err2.printStackTrace()
            }
        }

    Thread.setDefaultUncaughtExceptionHandler(globalExceptionHandler)

    Runtime.getRuntime().addShutdownHook(
        object : Thread() {
            override fun run() {
                try {
                    log("Shutdown hook called, cleaning up..", LogType.WARN)

                    Logger.cleanup()
                } catch (err: Exception) {
                    err.printStackTrace()
                }
            }
        }
    )

    // initiate the config before anything else happens
    loadVoyagerConfig()

    LoggerSettings.minDisplaySeverity =
        when (VOYAGER_CONFIG.minLogDisplay) {
            "TRACE" -> LogType.TRACE
            "DEBUG" -> LogType.DEBUG
            "INFO" -> LogType.INFO
            "WARN" -> LogType.WARN
            "ERROR" -> LogType.ERROR
            "FATAL" -> LogType.FATAL
            else -> LogType.INFO
        }.severity

    val mode = if (System.getenv().contains("development")) { "development" } else { "production" }
    log("Running Voyager in $mode mode on port 8765", LogType.INFO)

    
    log("Installing modules..", LogType.INFO)
    install(ContentNegotiation) {
        json()
    }

    log("Registering call interceptors..", LogType.INFO)
    intercept(ApplicationCallPipeline.Call) {
        
        // check if route is /status
        if (call.request.path() == "/status") {
            call.respond(
                HttpStatusCode.OK,
                "Voyager is up!"
            )
            return@intercept finish()
        }
        
        val apiKey = call.request.header("X-API-Key")

        if (apiKey == null || apiKey != VOYAGER_CONFIG.apiKey) {
            log("User tried to connect with invalid API Key: ${call.request.local}", LogType.DEBUG)
            call.respond(
                HttpStatusCode.Unauthorized,
                "Invalid API Key"
            )
            return@intercept finish()
        }
    }
    
    //createVoyagerSupabaseClient()
    connectToRedis()
    defineRedisSchema()

    configureDeploymentApi()

    log("Done in ${System.currentTimeMillis() - programStartTime}ms. Voyager is up!", LogType.INFO)
}

fun loadVoyagerConfig() {
    log("Getting voyager config from file..", LogType.INFO)
    val configFile = File("config.yml").also {
        if (!it.exists()) {
            log("Config file not found! Generating a template..", LogType.ERROR)
            it.createNewFile()
            it.writeText(
                Yaml.default.encodeToString(
                    VoyagerConfig()
                ).replace(Regex("isDevelopment: (true|false)"), "")
            )
            throw Exception("Config file not found. Generated a template")
        }
    }

    VOYAGER_CONFIG =
        Yaml.default.decodeFromString(
            VoyagerConfig.serializer(),
            configFile.readText(Charsets.UTF_8)
        )
    for (prop in VoyagerConfig::class.memberProperties) {
        if (prop.get(VOYAGER_CONFIG)?.equals("") == true) throw Exception("${prop.name} config not set")
    }
}

val VOYAGER_JSON = Json { 
    ignoreUnknownKeys = true
    isLenient = true
    encodeDefaults = true
    prettyPrint = true
}

lateinit var VOYAGER_CONFIG: VoyagerConfig
