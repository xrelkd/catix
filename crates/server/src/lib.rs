pub mod config;
mod error;
mod metrics;
mod web;

use std::{future::Future, net::SocketAddr, pin::Pin};

use futures::FutureExt;
use sigfinn::{ExitStatus, LifecycleManager, Shutdown};
use snafu::ResultExt;
use tokio::net::TcpListener;

pub use self::{
    config::Config,
    error::{Error, Result},
};
use crate::metrics::Metrics;

/// # Errors
///
/// This function will return an error if the server fails to start.
pub async fn serve_with_shutdown(
    Config { metrics: metrics_config, web: web_config }: Config,
) -> Result<()> {
    let lifecycle_manager = LifecycleManager::<Error>::new();

    let _handle =
        lifecycle_manager.spawn("Web server", create_web_server_future(web_config.listen_address));

    if metrics_config.enable {
        let metrics = Metrics::new()?;

        let _handle = lifecycle_manager.spawn(
            "Metrics server",
            create_metrics_server_future(metrics_config.listen_address, metrics),
        );
    }

    if let Ok(Err(err)) = lifecycle_manager.serve().await {
        tracing::error!("{err}");
        Err(err)
    } else {
        Ok(())
    }
}

fn create_web_server_future(
    listen_address: SocketAddr,
) -> impl FnOnce(Shutdown) -> Pin<Box<dyn Future<Output = ExitStatus<Error>> + Send>> {
    move |shutdown_signal| {
        async move {
            tracing::info!("Listening Web server on {listen_address}");

            let middleware_stack = tower::ServiceBuilder::new();

            let router = axum::Router::new()
                .merge(web::controller::api_v1_router())
                .layer(middleware_stack)
                .into_make_service_with_connect_info::<SocketAddr>();

            let maybe_listener =
                TcpListener::bind(&listen_address).await.context(error::BindWebServerSnafu);
            let listener = match maybe_listener {
                Ok(listener) => listener,
                Err(err) => {
                    return ExitStatus::FatalError(err);
                }
            };

            let result = axum::serve(listener, router)
                .with_graceful_shutdown(shutdown_signal)
                .await
                .context(error::ServeBindWebServerSnafu);

            match result {
                Ok(()) => {
                    tracing::info!("Stopped Web server gracefully");
                    ExitStatus::Success
                }
                Err(err) => ExitStatus::Error(err),
            }
        }
        .boxed()
    }
}

fn create_metrics_server_future<Metrics>(
    listen_address: SocketAddr,
    metrics: Metrics,
) -> impl FnOnce(Shutdown) -> Pin<Box<dyn Future<Output = ExitStatus<Error>> + Send>>
where
    Metrics: catix_metrics::Metrics + 'static,
{
    move |signal| {
        async move {
            tracing::info!("Listening metrics endpoint on {listen_address}");
            let result = catix_metrics::start_metrics_server(listen_address, metrics, signal).await;
            match result {
                Ok(()) => {
                    tracing::info!("Stopped Metrics server gracefully");
                    ExitStatus::Success
                }
                Err(err) => ExitStatus::Error(Error::from(err)),
            }
        }
        .boxed()
    }
}
