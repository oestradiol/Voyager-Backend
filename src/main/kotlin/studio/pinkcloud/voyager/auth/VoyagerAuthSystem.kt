package studio.pinkcloud.voyager.auth

import io.github.jan.supabase.gotrue.auth
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.pipeline.*
import studio.pinkcloud.voyager.supabase.SUPABASE_ATTRIBUTE

suspend fun PipelineContext<Unit, ApplicationCall>.authenticateWithSupabase() {
    /*
    val supabase = call.application.attributes[SUPABASE_ATTRIBUTE]
    val token = call.request.header("Authorization")?.removePrefix("Bearer ")

    // Validate the token with Supabase
    if (token != null) {
        try {
            supabase.auth.
        } catch (error: SupabaseError) {
            // Handle authentication error
            call.respond(HttpStatusCode.Unauthorized, "Invalid token")
            finish()
        }
    } else {
        // No token provided
        call.respond(HttpStatusCode.Unauthorized, "Token not provided")
        finish()
    }
    
     */
}