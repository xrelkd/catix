use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Could not create tokio runtime, error: {source}"))]
    InitializeTokioRuntime { source: std::io::Error },

    #[snafu(display("Error occurs while running lifecycle manager, error: {source}"))]
    LifecycleManager { source: sigfinn::Error },

    #[snafu(display("{source}"))]
    Application { source: catix_server::Error },

    #[snafu(display("{source}"))]
    Config {
        #[snafu(source(from(crate::config::Error, Box::new)))]
        source: Box<crate::config::Error>,
    },
}

impl From<catix_server::Error> for Error {
    fn from(source: catix_server::Error) -> Self { Self::Application { source } }
}

impl From<crate::config::Error> for Error {
    fn from(source: crate::config::Error) -> Self { Self::Config { source: Box::new(source) } }
}
