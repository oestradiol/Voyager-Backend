package studio.pinkcloud.voyager

import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.plugins.contentnegotiation.*
import kotlinx.serialization.json.Json
import studio.pinkcloud.voyager.deployment.IDeploymentSystem
import studio.pinkcloud.voyager.deployment.controller.configureDeployment

fun main() {
    embeddedServer(
        Netty,
        port = 8765,
        host = "0.0.0.0",
        module = Application::init
    ).start(wait = true)
}

fun Application.init() {
    install(ContentNegotiation) {
        json()
    }
    //createVoyagerSupabaseClient()
    
    configureDeployment()
    IDeploymentSystem.INSTANCE.load()
}

val VOYAGER_JSON = Json { 
    ignoreUnknownKeys = true
    isLenient = true
    encodeDefaults = true
    prettyPrint = true
}