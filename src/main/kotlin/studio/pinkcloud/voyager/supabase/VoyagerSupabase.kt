package studio.pinkcloud.voyager.supabase

import io.github.jan.supabase.SupabaseClient
import io.github.jan.supabase.createSupabaseClient
import io.github.jan.supabase.gotrue.Auth
import io.github.jan.supabase.gotrue.auth
import io.ktor.server.application.*
import io.ktor.util.*
import io.netty.handler.codec.AsciiHeadersEncoder.NewlineType

fun Application.createVoyagerSupabaseClient() {
    val supabase = createSupabaseClient(
        "https://<your-project>.supabase.co",
        "<your-anon-key>"
    ) {
        // Configuration for the supabase client.
    }
    
    attributes.put(SUPABASE_ATTRIBUTE, supabase)
}

val SUPABASE_ATTRIBUTE = SupabaseAttribute("voyager-supabase")
typealias SupabaseAttribute = AttributeKey<SupabaseClient>