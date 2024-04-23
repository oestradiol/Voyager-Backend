pub fn gen_traefik_labels(name: &str, host: &str, internal_port: u16) -> Vec<(String, String)> {
  vec![
    ("traefik.enable".to_string(), "true".to_string()),
    (
      format!("traefik.http.routers.voyager-{name}.entrypoints"),
      "web-secure".to_string(),
    ),
    (
      format!("traefik.http.routers.voyager-{name}.rule"),
      format!("Host(`{host}`)"),
    ),
    (
      format!("traefik.http.routers.voyager-{name}.service"),
      format!("voyager-{name}"),
    ),
    (
      format!(
        "traefik.http.services.voyager-{name}.loadbalancer.server.port"
      ),
      format!("{internal_port}"),
    ),
    // (
    //   format!("traefik.http.routers.voyager-{name}.tls"),
    //   "true".to_string(),
    // ),
  ]
}
