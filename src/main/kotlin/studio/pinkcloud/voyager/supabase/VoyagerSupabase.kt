package studio.pinkcloud.voyager.supabase

import io.github.jan.supabase.SupabaseClient
import io.github.jan.supabase.createSupabaseClient
import io.ktor.server.application.*
import io.ktor.util.*
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.utils.logging.log

fun Application.createVoyagerSupabaseClient() {
    log("Creating supabase client..")

    val supabase = createSupabaseClient(
        VOYAGER_CONFIG.supabaseUrl,
        VOYAGER_CONFIG.supabaseKey
    ) {
        // Configuration for the supabase client.
    }
    
    attributes.put(SUPABASE_ATTRIBUTE, supabase)

    log("Supabase client created")
}

val SUPABASE_ATTRIBUTE = SupabaseAttribute("voyager-supabase")
typealias SupabaseAttribute = AttributeKey<SupabaseClient>
