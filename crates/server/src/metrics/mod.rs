pub mod dbus;
pub mod grpc;

use catix_metrics::error;
use snafu::ResultExt;

#[derive(Clone, Debug)]
pub struct Metrics {
    registry: prometheus::Registry,
}

impl Metrics {
    pub fn new() -> Result<Self, catix_metrics::Error> {
        let registry = prometheus::Registry::new();

        // gRPC
        registry
            .register(Box::new(grpc::REQUESTS_TOTAL.clone()))
            .context(error::SetupMetricsSnafu)?;

        // D-Bus
        registry
            .register(Box::new(dbus::REQUESTS_TOTAL.clone()))
            .context(error::SetupMetricsSnafu)?;
        registry
            .register(Box::new(dbus::REQUEST_DURATION_SECONDS.clone()))
            .context(error::SetupMetricsSnafu)?;

        Ok(Self { registry })
    }
}

impl catix_metrics::Metrics for Metrics {
    fn gather(&self) -> Vec<prometheus::proto::MetricFamily> { self.registry.gather() }
}

#[cfg(test)]
mod tests {
    use crate::metrics::Metrics;

    #[test]
    fn test_new() { drop(Metrics::new().unwrap()); }
}
