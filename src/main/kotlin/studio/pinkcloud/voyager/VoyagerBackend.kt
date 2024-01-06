package studio.pinkcloud.voyager

import io.github.jan.supabase.SupabaseClient
import io.github.jan.supabase.createSupabaseClient
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.plugins.contentnegotiation.*
import studio.pinkcloud.voyager.deployment.configureDeployment
import studio.pinkcloud.voyager.supabase.createVoyagerSupabaseClient

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
}