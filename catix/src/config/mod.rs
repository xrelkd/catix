mod error;
mod log;
mod metrics;
mod web;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use snafu::ResultExt;

pub use self::{error::Error, log::LogConfig, metrics::MetricsConfig, web::WebConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(with = "http_serde_ext::uri::vec")]
    pub upstream_servers: Vec<http::Uri>,

    #[serde(default)]
    pub log: LogConfig,

    #[serde(default)]
    pub web: WebConfig,

    #[serde(default)]
    pub metrics: MetricsConfig,
}

impl Config {
    #[inline]
    pub fn default_path() -> PathBuf {
        [catix_base::PROJECT_CONFIG_DIR.to_path_buf(), PathBuf::from(catix_base::CONFIG_NAME)]
            .into_iter()
            .collect()
    }

    #[inline]
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let data = std::fs::read_to_string(&path)
            .context(error::OpenConfigSnafu { filename: path.as_ref().to_path_buf() })?;

        let config: Self = toml::from_str(&data)
            .context(error::ParseConfigSnafu { filename: path.as_ref().to_path_buf() })?;

        Ok(config)
    }

    #[inline]
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        match Self::load(&path) {
            Ok(config) => config,
            Err(err) => {
                tracing::warn!("Failed to read config file ({:?}), error: {err:?}", &path.as_ref(),);
                Self::default()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            upstream_servers: vec!["https://cache.nixos.org".parse().expect("valid uri")],
            log: LogConfig::default(),
            web: WebConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}

impl From<Config> for catix_server::Config {
    fn from(Config { web, metrics, upstream_servers, .. }: Config) -> Self {
        let metrics = catix_server::config::MetricsConfig::from(metrics);
        let web = catix_server::config::WebConfig::from(web);

        Self { upstream_servers, web, metrics }
    }
}
