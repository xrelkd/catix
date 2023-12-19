use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Error occurs while binding web server, error: {source}"))]
    BindWebServer { source: std::io::Error },

    #[snafu(display("Error occurs while serving web server, error: {source}"))]
    ServeBindWebServer { source: std::io::Error },

    #[snafu(display("{source}"))]
    Metrics { source: catix_metrics::Error },
}

impl From<catix_metrics::Error> for Error {
    fn from(source: catix_metrics::Error) -> Self { Self::Metrics { source } }
}
