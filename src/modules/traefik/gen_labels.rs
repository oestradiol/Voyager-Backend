pub fn gen_traefik_labels(name: String, host: String, internal_port: u16) -> Vec<(String, String)> {
  vec![
    ("traefik.enable".to_string(), "true".to_string()),
    (
      format!("traefik.http.routers.voyager-{}.entrypoints", name),
      "web,websecure".to_string(),
    ),
    (
      format!("traefik.http.routers.voyager-{}.rule", name),
      format!("Host(`{}`)", host),
    ),
    (
      format!("traefik.http.routers.voyager-{}.service", name),
      format!("voyager-{}-service", name),
    ),
    (
      format!(
        "traefik.http.services.voyager-{}-service.loadbalancer.server.port",
        name
      ),
      format!("{}", internal_port),
    ),
    (
      format!("traefik.http.routers.voyager-{}.tls", name),
      "true".to_string(),
    ),
  ]
}
