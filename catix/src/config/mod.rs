mod error;
mod log;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use snafu::ResultExt;

pub use self::{error::Error, log::LogConfig};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub log: LogConfig,
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
