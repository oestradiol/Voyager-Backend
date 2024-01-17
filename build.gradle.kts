val ktor_version: String by project
val kotlin_version: String by project
val logback_version: String by project
val supabase_version: String by project

plugins {
    kotlin("jvm") version "1.9.21"
    id("io.sentry.jvm.gradle") version "3.12.0"
    id("io.ktor.plugin") version "2.3.7"
    kotlin("plugin.serialization") version "1.9.22"
}

group = "studios.pinkcloud"
version = "0.0.1"

application {
    mainClass.set("studio.pinkcloud.voyager.VoyagerBackendKt")
}

repositories {
    mavenCentral()
    maven("https://jitpack.io")

}

dependencies {
    implementation("io.ktor:ktor-server-content-negotiation-jvm")
    implementation("io.ktor:ktor-server-core-jvm")
    implementation("io.ktor:ktor-serialization-kotlinx-json-jvm")
    implementation("io.ktor:ktor-server-netty-jvm")
    implementation("io.ktor:ktor-server-http-redirect:$ktor_version")
    implementation("ch.qos.logback:logback-classic:$logback_version")
    
    // supabase
    implementation("io.github.jan-tennert.supabase:gotrue-kt:$supabase_version")
    implementation("io.ktor:ktor-client-cio:$ktor_version")
    
    // github java api
    implementation("org.eclipse.jgit:org.eclipse.jgit:6.8.0.202311291450-r")
    
    // docker api for deploying
    implementation("com.github.docker-java:docker-java:3.3.4")
    implementation("com.github.docker-java:docker-java-transport-httpclient5:3.3.4")

    // discord for notifications
    implementation("club.minnced:discord-webhooks:0.8.4")
    
    // ktx-serialization for yaml for configuration
    implementation("com.charleskorn.kaml:kaml:0.56.0")
    implementation("io.ktor:ktor-server-http-redirect-jvm:2.3.7")

    testImplementation("io.ktor:ktor-server-tests-jvm")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit:$kotlin_version")

    // Sentry
    implementation("io.sentry:sentry:1.7.2")

    // Reflection
    implementation(kotlin("reflect"))

    // Redis
    implementation("redis.clients:jedis:5.0.0")

    // Console text styling
    implementation("com.github.ajalt.mordant:mordant:2.2.0")

    // Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")

    // json serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.2")

    // Arrow-kt
    implementation("io.arrow-kt:arrow-core:1.2.0")
    implementation("io.arrow-kt:arrow-fx-coroutines:1.2.0")

    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.3.2")
}

kotlin {
    jvmToolchain(17)
}

tasks.register("runDev") {
    doFirst {
        tasks.run.configure {
            application.applicationDefaultJvmArgs = listOf("-Dio.ktor.development=true")
            environment(mapOf("development" to 1))
        }
    }
    finalizedBy("run")
}

tasks.register("buildWithSentry") {
    /* I dont know 9if this is working rn :kiss:
    doFirst {
        tasks.build.configure {
            sentry {
                // Generates a JVM (Java, Kotlin, etc.) source bundle and uploads your source code to Sentry.
                // This enables source context, allowing you to see your source
                // code as part of your stack traces in Sentry.
                includeSourceContext = true

                org = "pinkcloud"
                projectName = "voyager-backend"
                authToken = System.getenv("SENTRY_AUTH_TOKEN")
            }
        }
    }
    
     */
    finalizedBy("build")
}
