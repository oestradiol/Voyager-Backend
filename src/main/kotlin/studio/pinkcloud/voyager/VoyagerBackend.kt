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
import studio.pinkcloud.voyager.deployment.AbstractDeploymentSystem
import studio.pinkcloud.voyager.deployment.controller.configurePreviewDeployment
import studio.pinkcloud.voyager.deployment.controller.configureProductionDeployment
import java.io.File

fun main() {
    embeddedServer(
        Netty,
        port = 8765,
        host = "0.0.0.0",
        module = Application::init
    ).start(wait = true)
}

fun Application.init() {
    // initiate the config before anything else happens
    val configFile = File("config.yml").also { 
        if (!it.exists()) {
            it.createNewFile()
            it.writeText(
                Yaml.default.encodeToString(
                    VoyagerConfig()
                )
            )
        }
    }
    
    VOYAGER_CONFIG = 
        Yaml.default.decodeFromString(
            VoyagerConfig.serializer(), 
            configFile.readText(Charsets.UTF_8)
        )
    
    install(ContentNegotiation) {
        json()
    }

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

lateinit var VOYAGER_CONFIG: VoyagerConfig