package studio.pinkcloud.voyager.redis

import redis.clients.jedis.*
import redis.clients.jedis.commands.ProtocolCommand
import redis.clients.jedis.util.SafeEncoder
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.utils.logging.*
import studio.pinkcloud.voyager.RESOURCES_DIR
import io.ktor.server.application.Application
import java.io.File
import java.io.FileNotFoundException
import java.util.Scanner
import com.sun.jna.StringArray
import studio.pinkcloud.voyager.utils.VoyagerResponse

lateinit var redisClient: JedisPooled

fun connectToRedis() {
    log("Connecting to redis..", LogType.INFO)
    try {
        redisClient = JedisPooled(VOYAGER_CONFIG.redisUrl, VOYAGER_CONFIG.redisPort)
        if (!redisClient.ping().equals("PONG")) throw Exception("Redis client created but PING failed")
    } catch (err: Exception) {
        log("Failed to connect to redis", LogType.FATAL)
        throw err
    }
    log("Connected to redis successfully", LogType.INFO)
}

fun redisGetCommandName(command: String): String {
    return command.substringBefore(' ')
}

fun redisGetCommandArgsStr(command: String): String {
    return command.substringAfter(' ').trim()
}

fun redisGetCommandArgsArray(command: String): Array<String> {
    return redisGetCommandArgsStr(command).split(" ").toTypedArray()
}

fun formatRedisCommand(command: String): Pair<String, Array<String>> {
    return Pair(redisGetCommandName(command), redisGetCommandArgsArray(command))
}

fun redisSendCommand(command: String): Any {
    val formatted = formatRedisCommand(command)
    return redisClient.sendCommand(object : ProtocolCommand {
                                override fun getRaw(): ByteArray {
                                    return SafeEncoder.encode(formatted.first)
                                }
                            }, *formatted.second)
}

fun redisSendBlockingCommand(command: String): Any {
    val formatted = formatRedisCommand(command)
    return redisClient.sendBlockingCommand(object : ProtocolCommand {
                                override fun getRaw(): ByteArray {
                                    return SafeEncoder.encode(formatted.first)
                                }
                            }, *formatted.second)

}

fun defineRedisSchema() {
    log("Defining redis schema..", LogType.INFO)

    val redisSchema = VoyagerResponse::class.java.getResource("/redis-schema.txt")?.readText()
        ?: throw FileNotFoundException("redis-schema.txt not found")
    
    try {
        val formattedSchemaSplit = redisSchema
            .replace(Regex("//[^\\n]*"), "") // removing comments
            .replace(Regex("\\s+"), " ") // removing extra whitespace
            .trim()
            .split("---") // splitting commands

        log("Commands in redis-schema.txt:", LogType.DEBUG)
        for (command in formattedSchemaSplit) {
            if (command == "") continue
            log("", LogType.DEBUG)
            log("Processing command:", LogType.DEBUG)
            log(command, LogType.DEBUG)
            try {
                redisSendBlockingCommand(command)
            } catch (err: Exception) {
                if (!err.message.equals("Index already exists")) {
                    log("Command failed: ${err.message}. It is unrecoverable, aborting..", LogType.FATAL)
                    throw err
                }

                log("Index already exists.", LogType.WARN)

                if (VOYAGER_CONFIG.forceRedisSync) {
                    log("forceRedisSync is set to true, dropping old index", LogType.WARN)
                    redisSendBlockingCommand("FT.DROP " + redisGetCommandArgsStr(command).substringBefore(' '))
                    redisSendBlockingCommand(command)
                    log("Success!", LogType.INFO)
                    return
                }

                log("forceRedisSync is set to false, ignoring..", LogType.WARN)
            }
        }

        log("Success!", LogType.INFO)

    } catch (err: Exception) {
        log("Redis schema defining failed: ${err.message}", LogType.ERROR)
        if (VOYAGER_CONFIG.forceRedisSync) {
            log("forceRedisSync is set to true, aborting..", LogType.FATAL)
            throw err
        }
    }
}
