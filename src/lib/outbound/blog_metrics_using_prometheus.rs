use crate::domain::blog::ports::BlogMetrics;

/// An unimplemented example of an adapter to [BlogMetrics].
#[derive(Debug, Clone)]
pub struct BlogMetricsUsingPrometheus;

impl BlogMetricsUsingPrometheus {
    pub fn new() -> Self {
        Self
    }
}

impl BlogMetrics for BlogMetricsUsingPrometheus {
    async fn record_author_creation_success(&self) {}

    async fn record_author_creation_failure(&self) {}
}
