pub mod model;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use directories::ProjectDirs;
use lazy_static::lazy_static;

pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref PROJECT_SEMVER: semver::Version = semver::Version::parse(PROJECT_VERSION)
        .unwrap_or(semver::Version {
            major: 0,
            minor: 0,
            patch: 0,
            pre: semver::Prerelease::EMPTY,
            build: semver::BuildMetadata::EMPTY
        });
    pub static ref DEFAULT_HTTP_USER_AGENT: String =
        format!("{PROJECT_NAME_WITH_INITIAL_CAPITAL}/{PROJECT_VERSION}");
}

pub const PROJECT_NAME: &str = "catix";
pub const PROJECT_NAME_WITH_INITIAL_CAPITAL: &str = "Catix";
pub const CONFIG_NAME: &str = "catix.toml";

pub const DEFAULT_WEB_PORT: u16 = 17000;
pub const DEFAULT_WEB_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

pub const DEFAULT_METRICS_PORT: u16 = 17001;
pub const DEFAULT_METRICS_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

lazy_static::lazy_static! {
pub static ref PROJECT_CONFIG_DIR: PathBuf = ProjectDirs::from("", PROJECT_NAME, PROJECT_NAME)
            .expect("Creating `ProjectDirs` should always success")
            .config_dir()
            .to_path_buf();
}
