
import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import io.ktor.server.testing.*
import kotlinx.coroutines.async
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.deployment.model.DeploymentState
import studio.pinkcloud.voyager.deployment.view.DeployResponse
import studio.pinkcloud.voyager.init
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.utils.logging.LoggerSettings
import studio.pinkcloud.voyager.utils.logging.log
import kotlin.random.Random
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

class VoyagerBackendTests {

    companion object {
        const val ALPHABET: String =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

        fun genRandomString(len: Int): String {
            var result = ""
            for (i in 0..len) {
                result += ALPHABET.get(Random.nextInt(0, ALPHABET.length))
            }

            return result
        }

        fun genRandomDeployment(): Deployment {
            return Deployment(
                genRandomString(10),
                genRandomString(10),
                Random.nextInt(-1000, 1000),
                genRandomString(10),
                DeploymentMode.PREVIEW,
                genRandomString(10),
                DeploymentState.STOPPED,
                genRandomString(10),
                Random.nextLong(1000000000)
            )
        }
    }

    @Test
    fun initConfigs() = testApplication {
        application {
            init()
            LoggerSettings.loggerThreadDelay = 1
        }
    }

    @Test
    fun testRedisConnection() = testApplication {
        application {
            init()
            LoggerSettings.loggerThreadDelay = 1
        }
        assertEquals(redisClient.ping(), "PONG")
        val strings = Array(100) { _: Int -> genRandomString(100) }
        for (i in 0..99) {
            redisClient.set("testvar:$i", strings[i])
        }
        for (i in 0..99) {
            val raw = redisClient.get("testvar:$i")
            assertEquals(String(raw.toByteArray(), Charsets.UTF_8), String(strings[i].toByteArray(), Charsets.UTF_8))
        }
    }


    @Test
    fun testDeploymentEncoding() = testApplication {
        for (i in 0..100) {
            val deployment = genRandomDeployment()
            Thread.sleep(5)
            assertEquals(deployment, Json.decodeFromString<Deployment>(Json.encodeToString(Deployment.serializer(), deployment)))
        }
    }

    @Test
    fun testDeploymentOnRedis() = testApplication {
        application {
            init()
            LoggerSettings.loggerThreadDelay = 1
        }
        runBlocking {
            assertEquals(redisClient.ping(), "PONG")
            val deployments = Array(100) { _: Int -> genRandomDeployment() }
            for (i in 0..99) {
                async { deployments[i].save() }.await()
            }
            val deploymentsFound = async { Deployment.findAll() }.await()
            for (found in deploymentsFound) {
                for (depl in deployments) {
                    if (depl.id != found.id) continue

                    assertEquals(
                        String(found.toString().toByteArray(), Charsets.UTF_8),
                        String(depl.toString().toByteArray(), Charsets.UTF_8)
                    )
                }
            }
            for (i in 0..99) {
                val found = async { Deployment.findById(deployments[i].id) }.await()
                assertEquals(
                    String(found.toString().toByteArray(), Charsets.UTF_8),
                    String(deployments[i].toString().toByteArray(), Charsets.UTF_8)
                )
                found?.deleteFromRedis()
            }
            for (i in 0..99) {
                val found = async { Deployment.findById(deployments[i].id) }.await()
                assertEquals(found, null)
            }
        }
    }

    @Test
    fun testDeployment() = testApplication {
        application {
            init()
            LoggerSettings.loggerThreadDelay = 1
        }
        runBlocking {
            val deployRes = client.post("/deployment/deploy") {
                contentType(ContentType.Application.Json)
                header("X-API-Key", VOYAGER_CONFIG.apiKey)
                header("X-Repo-URL", "atomoxetine/mock-server")
                header("X-Mode", "preview")
            }

            log(deployRes.toString())

            assertEquals(deployRes.status, HttpStatusCode.OK)

            val deployResBody = Json.decodeFromString<DeployResponse>(deployRes.bodyAsText(Charsets.UTF_8))
            val id = deployResBody.id!!
            val deployment = Deployment.findById(id)

            assertNotNull(deployment)

            val port = deployment.port
            val pingRes = HttpClient().get("http://localhost:$port/ping")

            assertEquals(pingRes.status, HttpStatusCode.OK)
            assertEquals(pingRes.bodyAsText(), "pong")

            val stopRes = client.post("/deployment/$id/stop") {
                contentType(ContentType.Application.Json)
                header("X-API-Key", VOYAGER_CONFIG.apiKey)
            }

            assertEquals(stopRes.status, HttpStatusCode.OK)

        }
    }
}
