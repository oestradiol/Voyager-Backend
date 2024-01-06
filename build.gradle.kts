val ktor_version: String by project
val kotlin_version: String by project
val logback_version: String by project
val supabase_version: String by project

plugins {
    kotlin("jvm") version "1.9.21"
    id("io.sentry.jvm.gradle") version "3.12.0"
    id("io.ktor.plugin") version "2.3.7"
    id("org.jetbrains.kotlin.plugin.serialization") version "1.9.22"
}

group = "studios.pinkcloud"
version = "0.0.1"

sentry {
    // Generates a JVM (Java, Kotlin, etc.) source bundle and uploads your source code to Sentry.
    // This enables source context, allowing you to see your source
    // code as part of your stack traces in Sentry.
    includeSourceContext = true

    org = "pinkcloud"
    projectName = "voyager-backend"
    authToken = System.getenv("SENTRY_AUTH_TOKEN")
}

application {
    mainClass.set("pinkcloud.studio.ApplicationKt")

    val isDevelopment: Boolean = project.ext.has("development")
    applicationDefaultJvmArgs = listOf("-Dio.ktor.development=$isDevelopment")
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("io.ktor:ktor-server-content-negotiation-jvm")
    implementation("io.ktor:ktor-server-core-jvm")
    implementation("io.ktor:ktor-serialization-kotlinx-json-jvm")
    implementation("io.ktor:ktor-server-netty-jvm")
    implementation("ch.qos.logback:logback-classic:$logback_version")
    
    // supabase shit
    implementation("io.github.jan-tennert.supabase:gotrue-kt:$supabase_version")
    implementation("io.ktor:ktor-client-cio:$ktor_version")
    
    // github java api
    implementation("org.kohsuke:github-api:1.318")
    
    testImplementation("io.ktor:ktor-server-tests-jvm")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit:$kotlin_version")
}

kotlin {
    jvmToolchain(17)
}