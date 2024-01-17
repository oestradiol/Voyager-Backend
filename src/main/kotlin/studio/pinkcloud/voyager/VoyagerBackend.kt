package studio.pinkcloud.voyager

import com.charleskorn.kaml.Yaml
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.httpsredirect.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import studio.pinkcloud.voyager.config.VoyagerConfig
import studio.pinkcloud.voyager.deployment.controller.configurePreviewDeployment
import studio.pinkcloud.voyager.deployment.controller.configureProductionDeployment
import studio.pinkcloud.voyager.redis.connectToRedis
import studio.pinkcloud.voyager.redis.defineRedisSchema
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.LoggerFileWriter
import studio.pinkcloud.voyager.utils.logging.LoggerSettings
import studio.pinkcloud.voyager.utils.logging.log
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

    val mode = if (System.getenv().contains("development")) {
        "development"
    } else {
        "production"
    }
    log("Running Voyager in $mode mode on port 8765", LogType.INFO)


    log("Installing modules", LogType.INFO)
    install(ContentNegotiation) {
        json()
    }

    install(HttpsRedirect)
    
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
    log("Loading voyager config..", LogType.INFO)

    var toDecodeYaml = ""

    log("Getting voyager config from env variables..", LogType.INFO)

    VoyagerConfig::class.memberProperties.forEach {
        val value = System.getenv(it.name) ?: return@forEach

        toDecodeYaml += "${it.name}: \"$value\"\n"
    }

    log("Configs got from environment: ", LogType.INFO)
    log("\n$toDecodeYaml", LogType.INFO)

    VOYAGER_CONFIG =
        Yaml.default.decodeFromString(
            VoyagerConfig.serializer(),
            toDecodeYaml
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
