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
import io.ktor.network.sockets.connect
import io.ktor.server.routing.*
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import kotlinx.coroutines.runBlocking
import studio.pinkcloud.voyager.config.VoyagerConfig
import studio.pinkcloud.voyager.deployment.controller.configurePreviewDeployment
import studio.pinkcloud.voyager.deployment.controller.configureProductionDeployment
import java.io.File
import studio.pinkcloud.voyager.utils.logging.*
import studio.pinkcloud.voyager.redis.*
import studio.pinkcloud.voyager.redis.connectToRedis
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
    runBlocking { LoggerFileWriter.load() }

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
    log("Running Voyager in ${mode} mode on port 8765", LogType.INFO)

    
    log("Installing modules", LogType.INFO)
    install(ContentNegotiation) {
        json()
    }

    log("Registering interceptors", LogType.INFO)
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

    configurePreviewDeployment()
    configureProductionDeployment()

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
        if (prop.get(VOYAGER_CONFIG)?.equals("") ?: false) throw Exception("${prop.name} config not set")
    }
}

val VOYAGER_JSON = Json { 
    ignoreUnknownKeys = true
    isLenient = true
    encodeDefaults = true
    prettyPrint = true
}

const val RESOURCES_DIR = "src/main/resources"

lateinit var VOYAGER_CONFIG: VoyagerConfig
