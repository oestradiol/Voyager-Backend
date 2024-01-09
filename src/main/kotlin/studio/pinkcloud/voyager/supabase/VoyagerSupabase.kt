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
        "https://qyyxkkbneewcycarhrey.supabase.co",
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InF5eXhra2JuZWV3Y3ljYXJocmV5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDQ0ODU1MjAsImV4cCI6MjAyMDA2MTUyMH0.E8Mo1D9bsdBj6GWoRuRn5k9rBhKUBExO-LBWk_SQSTA"
    ) {
        // Configuration for the supabase client.
    }
    
    attributes.put(SUPABASE_ATTRIBUTE, supabase)
}

val SUPABASE_ATTRIBUTE = SupabaseAttribute("voyager-supabase")
typealias SupabaseAttribute = AttributeKey<SupabaseClient>