package studio.pinkcloud.voyager.utils
import studio.pinkcloud.voyager.utils.logging.*

object PortFinder {
    fun findFreePort(): Int {
        log("Finding free port..", LogType.DEBUG)
        val socket = java.net.ServerSocket(0)

        socket.use {
            log("Found free port: ${socket.localPort}", LogType.DEBUG)
            return socket.localPort
        }
    }
}
