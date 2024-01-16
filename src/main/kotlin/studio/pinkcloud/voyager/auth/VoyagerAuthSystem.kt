package studio.pinkcloud.voyager.auth

import io.github.jan.supabase.gotrue.auth
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.pipeline.*
import studio.pinkcloud.voyager.supabase.SUPABASE_ATTRIBUTE
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

suspend fun PipelineContext<Unit, ApplicationCall>.authenticateWithSupabase() {
    
    log("Authenticating application call with Supabase", LogType.DEBUG)

    val supabase = call.application.attributes[SUPABASE_ATTRIBUTE]
    val token = call.request.header("Authorization")?.removePrefix("Bearer ")

    // Validate the token with Supabase
    if (token != null) {
        try { 
            val user = supabase.auth.retrieveUser(token)

            log("Token is valid", LogType.DEBUG)
            
            user.id
        } catch (error: RuntimeException) {
            // Handle authentication error
            log("Token is invalid", LogType.DEBUG)
            call.respond(HttpStatusCode.Unauthorized, "Invalid token")
            finish()
        }
    } else {
        log("Token is null", LogType.DEBUG)
        // No token provided
        call.respond(HttpStatusCode.Unauthorized, "Token not provided")
        finish()
    }
}
