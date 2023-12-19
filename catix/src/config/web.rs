use std::net::{IpAddr, SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WebConfig {
    #[serde(default = "WebConfig::default_host")]
    pub host: IpAddr,

    #[serde(default = "WebConfig::default_port")]
    pub port: u16,
}

impl WebConfig {
    #[inline]
    pub const fn socket_address(&self) -> SocketAddr { SocketAddr::new(self.host, self.port) }

    #[inline]
    pub const fn default_host() -> IpAddr { catix_base::DEFAULT_WEB_HOST }

    #[inline]
    pub const fn default_port() -> u16 { catix_base::DEFAULT_WEB_PORT }
}

impl Default for WebConfig {
    fn default() -> Self { Self { host: Self::default_host(), port: Self::default_port() } }
}

impl From<WebConfig> for catix_server::config::WebConfig {
    fn from(config: WebConfig) -> Self { Self { listen_address: config.socket_address() } }
}
