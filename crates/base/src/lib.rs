pub mod model;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::LazyLock,
};

use directories::ProjectDirs;

pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub static PROJECT_SEMVER: LazyLock<semver::Version> = LazyLock::new(|| {
    semver::Version::parse(PROJECT_VERSION).unwrap_or(semver::Version {
        major: 0,
        minor: 0,
        patch: 0,
        pre: semver::Prerelease::EMPTY,
        build: semver::BuildMetadata::EMPTY,
    })
});

pub static DEFAULT_HTTP_USER_AGENT: LazyLock<String> =
    LazyLock::new(|| format!("{PROJECT_NAME_WITH_INITIAL_CAPITAL}/{PROJECT_VERSION}"));

pub const PROJECT_NAME: &str = "catix";
pub const PROJECT_NAME_WITH_INITIAL_CAPITAL: &str = "Catix";
pub const CONFIG_NAME: &str = "catix.toml";

pub const DEFAULT_WEB_PORT: u16 = 17000;
pub const DEFAULT_WEB_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

pub const DEFAULT_METRICS_PORT: u16 = 17001;
pub const DEFAULT_METRICS_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

pub static PROJECT_CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    ProjectDirs::from("", PROJECT_NAME, PROJECT_NAME)
        .expect("Creating `ProjectDirs` should always success")
        .config_dir()
        .to_path_buf()
});
