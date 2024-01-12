use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub upstream_servers: Vec<http::Uri>,

    pub web: WebConfig,

    pub metrics: MetricsConfig,
}

#[derive(Clone, Debug)]
pub struct WebConfig {
    pub listen_address: SocketAddr,
}

#[derive(Clone, Debug)]
pub struct MetricsConfig {
    pub enable: bool,

    pub listen_address: SocketAddr,
}
