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
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import studio.pinkcloud.voyager.config.VoyagerConfig
import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import studio.pinkcloud.voyager.deployment.controller.configurePreviewDeployment
import studio.pinkcloud.voyager.deployment.controller.configureProductionDeployment
import java.io.File
import studio.pinkcloud.voyager.utils.logging.*
import studio.pinkcloud.voyager.redis.*
import studio.pinkcloud.voyager.redis.connectToRedis
import kotlin.reflect.full.memberProperties

fun main() {
    LoggerFileWriter.load()

    try {
        embeddedServer(
            Netty,
            port = 8765,
            host = "0.0.0.0",
            module = Application::init
        ).start(wait = true)
    } catch (err: Exception) {
        log(err)
        throw Exception()
    }
}

fun Application.init() {
    val mode = if (System.getenv().contains("development")) { "development" } else { "production" }
    log("Running Voyager in ${mode}", LogType.INFORMATION)

    // initiate the config before anything else happens
    log("Getting voyager config from file..", LogType.INFORMATION)
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

    
    log("Installing modules", LogType.INFORMATION)
    install(ContentNegotiation) {
        json()
    }

    log("Registering interceptors", LogType.INFORMATION)
    intercept(ApplicationCallPipeline.Call) {
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
    AbstractDeploymentSystem.PRODUCTION_INSTANCE.load()
    AbstractDeploymentSystem.PREVIEW_INSTANCE.load()

}

val VOYAGER_JSON = Json { 
    ignoreUnknownKeys = true
    isLenient = true
    encodeDefaults = true
    prettyPrint = true
}

const val RESOURCES_DIR = "src/main/resources"

lateinit var VOYAGER_CONFIG: VoyagerConfig
