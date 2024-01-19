
import io.ktor.server.testing.*
import kotlinx.coroutines.async
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import studio.pinkcloud.voyager.deployment.model.Deployment
import studio.pinkcloud.voyager.deployment.model.DeploymentMode
import studio.pinkcloud.voyager.deployment.model.DeploymentState
import studio.pinkcloud.voyager.loadVoyagerConfig
import studio.pinkcloud.voyager.redis.connectToRedis
import studio.pinkcloud.voyager.redis.defineRedisSchema
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.utils.logging.Logger
import kotlin.random.Random
import kotlin.test.Test
import kotlin.test.assertEquals

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
    fun testRedisConnection() = testApplication {
        loadVoyagerConfig()
        connectToRedis()
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
        loadVoyagerConfig()
        for (i in 0..100) {
            val deployment = genRandomDeployment()
            Thread.sleep(5)
            assertEquals(deployment, Json.decodeFromString<Deployment>(Json.encodeToString(Deployment.serializer(), deployment)))
        }
    }

    @Test
    fun testDeploymentOnRedis() = testApplication {
        runBlocking {
            loadVoyagerConfig()
            connectToRedis()
            defineRedisSchema()
            Logger.load(1)
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
}
