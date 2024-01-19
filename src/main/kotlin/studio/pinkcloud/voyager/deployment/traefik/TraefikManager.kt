package studio.pinkcloud.voyager.deployment.traefik

import studio.pinkcloud.voyager.utils.logging.LogType
import studio.pinkcloud.voyager.utils.logging.log

object TraefikManager {
    fun genTraefikLabels(name: String, host: String, internalPort: Int): Map<String, String> {
        log("Generating traefik labels..", LogType.DEBUG)

        return mapOf(
            "traefik.enable" to "true",
            "traefik.http.routers.voyager-$name.entrypoints" to "http,https",
            "traefik.http.routers.voyager-$name.rule" to "Host(`$host`)",
            "traefik.http.routers.voyager-$name.service" to "voyager-$name-service",
            "traefik.http.services.voyager-$name-service.loadbalancer.server.port" to "$internalPort",
            "traefik.http.routers.voyager-$name.tls" to "true",
        )
    }
}