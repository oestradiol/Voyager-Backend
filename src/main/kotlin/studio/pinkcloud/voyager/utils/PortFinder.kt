package studio.pinkcloud.voyager.utils

object PortFinder {
    fun findFreePort(): Int {
        val socket = java.net.ServerSocket(0)
        
        socket.use {
            return socket.localPort
        }
    }
}