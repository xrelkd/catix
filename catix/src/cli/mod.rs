use std::{io::Write, net::IpAddr, path::PathBuf};

use clap::{CommandFactory, Parser, Subcommand};
use snafu::ResultExt;
use tokio::runtime::Runtime;

use crate::{config::Config, error, error::Error, shadow};

#[derive(Parser)]
#[command(
    name = catix_base::PROJECT_NAME,
    author,
    version,
    long_version = shadow::CLAP_LONG_VERSION,
    about,
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,

    #[arg(long = "host", help = "The address to bind to")]
    host: Option<IpAddr>,

    #[arg(long = "port", short = 'p', help = "The port to bind to")]
    port: Option<u16>,

    #[arg(long = "enable-metrics", help = "Enable metrics")]
    enable_metrics: bool,

    #[arg(long = "metrics-host", help = "The address of metrics to bind to")]
    metrics_host: Option<IpAddr>,

    #[arg(long = "metrics-port", help = "The port of metrics to bind to")]
    metrics_port: Option<u16>,

    #[arg(
        long = "upstream-servers",
        short = 's',
        aliases = ["substituters"],
        help = "Specify upstream servers"
    )]
    upstream_servers: Vec<http::Uri>,

    #[arg(
        long = "extra-upstream-servers",
        short = 'e',
        aliases = ["extra-substituters"],
        help = "Specify extra upstream servers"
    )]
    extra_upstream_servers: Vec<http::Uri>,

    #[arg(long = "log-level", env = "CATIX_LOG_LEVEL", help = "Specify a log level")]
    log_level: Option<tracing::Level>,

    #[arg(
        long = "config",
        short = 'C',
        env = "CATIX_CONFIG_FILE_PATH",
        help = "Specify a configuration file"
    )]
    config_file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Print version information")]
    Version,

    #[clap(about = "Output shell completion code for the specified shell (bash, zsh, fish)")]
    Completions { shell: clap_complete::Shell },

    #[clap(about = "Output default configuration")]
    DefaultConfig,
}

impl Default for Cli {
    fn default() -> Self { Self::parse() }
}

impl Cli {
    pub fn run(self) -> Result<(), Error> {
        let Self {
            commands,
            host,
            port,
            enable_metrics,
            metrics_host,
            metrics_port,
            upstream_servers,
            extra_upstream_servers,
            log_level,
            config_file,
        } = self;

        match commands {
            Some(Commands::Completions { shell }) => {
                let mut app = Self::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
                return Ok(());
            }
            Some(Commands::DefaultConfig) => {
                let config_text =
                    toml::to_string_pretty(&Config::default()).expect("Config is serializable");
                std::io::stdout()
                    .write_all(config_text.as_bytes())
                    .expect("Failed to write to stdout");
                return Ok(());
            }
            _ => {}
        }

        let mut config = Config::load_or_default(config_file.unwrap_or_else(Config::default_path));

        if let Some(host) = host {
            config.web.host = host;
        }

        if let Some(port) = port {
            config.web.port = port;
        }

        if enable_metrics {
            config.metrics.enable = true;
        }

        if let Some(host) = metrics_host {
            config.metrics.host = host;
        }

        if let Some(port) = metrics_port {
            config.metrics.port = port;
        }

        if !upstream_servers.is_empty() {
            config.upstream_servers = upstream_servers;
        }

        config.upstream_servers.extend(extra_upstream_servers);

        if let Some(log_level) = log_level {
            config.log.level = log_level;
        }

        config.log.registry();

        Runtime::new().context(error::InitializeTokioRuntimeSnafu)?.block_on(async move {
            match commands {
                Some(Commands::Completions { .. } | Commands::DefaultConfig) => {
                    unreachable!("these commands should be handled previously");
                }
                Some(Commands::Version) => {
                    std::io::stdout()
                        .write_all(Self::command().render_long_version().as_bytes())
                        .expect("Failed to write to stdout");
                    Ok(())
                }
                None => {
                    let config = catix_server::Config::from(config);
                    catix_server::serve_with_shutdown(config).await?;
                    Ok(())
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::{Cli, Commands};

    #[test]
    fn test_command_simple() {
        if let Some(Commands::Version { .. }) =
            Cli::parse_from(["program_name", "version"]).commands
        {
            // everything is good.
        } else {
            panic!();
        }
    }
}
