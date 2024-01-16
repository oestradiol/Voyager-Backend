package studio.pinkcloud.voyager.redis

import redis.clients.jedis.JedisPooled
import redis.clients.jedis.commands.ProtocolCommand
import redis.clients.jedis.util.SafeEncoder
import studio.pinkcloud.voyager.VOYAGER_CONFIG
import studio.pinkcloud.voyager.utils.VoyagerResponse
import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log
import java.io.FileNotFoundException

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
    log("Getting redis command name for command: $command", LogType.TRACE)
    return command.substringBefore(' ')
}

fun redisGetCommandArgsStr(command: String): String {
    log("Getting redis command arguments string for command: $command", LogType.TRACE)
    return command.substringAfter(' ').trim()
}

fun redisGetCommandArgsArray(command: String): Array<String> {
    log("Getting redis command arguments array for command: $command", LogType.TRACE)
    return redisGetCommandArgsStr(command).split(" ").toTypedArray()
}

fun formatRedisCommand(command: String): Pair<String, Array<String>> {
    log("Formatting redis command: $command", LogType.TRACE)
    return Pair(redisGetCommandName(command), redisGetCommandArgsArray(command))
}

fun redisSendCommand(command: String): Any {
    log("Sending redis command: $command", LogType.DEBUG)
    val formatted = formatRedisCommand(command)
    log("Formatted redis command: $formatted, original: $command", LogType.TRACE)
    return redisClient.sendCommand(object : ProtocolCommand {
                                override fun getRaw(): ByteArray {
                                    return SafeEncoder.encode(formatted.first)
                                }
                            }, *formatted.second)
}

fun redisSendBlockingCommand(command: String): Any {
    log("Sending redis blocking command: $command", LogType.DEBUG)
    val formatted = formatRedisCommand(command)
    log("Formatted redis command: $formatted, original: $command", LogType.TRACE)
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
        log("Formatting redis schema", LogType.DEBUG)
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
                    log("Redis index updated", LogType.WARN)
                    return
                }

                log("forceRedisSync is set to false, ignoring..", LogType.WARN)
            }
        }

        log("Redis is in sync", LogType.INFO)

    } catch (err: Exception) {
        log("Redis schema defining failed: ${err.message}", LogType.ERROR)
        if (VOYAGER_CONFIG.forceRedisSync) {
            log("forceRedisSync is set to true, aborting..", LogType.FATAL)
            throw err
        }
    }
}
