import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.testing.*
import kotlin.test.*
import kotlin.random.Random
import studio.pinkcloud.voyager.utils.logging.*
import studio.pinkcloud.voyager.redis.connectToRedis
import studio.pinkcloud.voyager.redis.redisClient
import studio.pinkcloud.voyager.deployment.model.*
import studio.pinkcloud.voyager.loadVoyagerConfig
import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import okio.utf8Size

class VoyagerBackendTests {

    companion object {
        const val ALPHABET: String =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%¨&*()\\\n\r\t\"'><,.;:?/[]{}`´~^=+-_®°ŧ←↓→øþæßðđŋħʒĸłç«»©„“”µ•·°ºª§¬🥺❤😁️"

        fun genRandomString(len: Int): String {
            var result = ""
            for (i in 0..len) {
                result += ALPHABET.get(Random.nextInt(0, ALPHABET.length))
            }

            return result
        }

        fun genRandomDeployment(): Deployment {
            return Deployment(
                genRandomString(100),
                Random.nextInt(-1000, 1000),
                genRandomString(100),
                genRandomString(100),
                DeploymentMode.PREVIEW,
                genRandomString(100),
                DeploymentState.STOPPED,
                Random.nextLong(1000000000)
            )
        }
    }

/* */
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
    */


    @Test
    fun testDeploymentEncoding() = testApplication {
        loadVoyagerConfig()
        for (i in 0..100) {
            val deployment = genRandomDeployment()
            Thread.sleep(5)
            assertEquals(deployment, Json.decodeFromString<Deployment>(Json.encodeToString(Deployment.serializer(), deployment)))
        }
    }

/*
    @Test
    fun testDeploymentOnRedis() = testApplication {
        loadVoyagerConfig()
        connectToRedis()
        assertEquals(redisClient.ping(), "PONG")
        val deployments = Array(100) { _: Int -> genRandomDeployment() }
        for (i in 0..99) {
            deployments[i].save()
        }
        for (i in 0..99) {
            val found = Deployment.find(deployments[i].deploymentKey)
            assertEquals(String(found.toString().toByteArray(), Charsets.UTF_8), String(deployments[i].toString().toByteArray(), Charsets.UTF_8))
            found?.deleteFromRedis()
        }
        for (i in 0..99) {
            val found = Deployment.find(deployments[i].deploymentKey)
            assertEquals(found, null)
        }
    }
    */
}