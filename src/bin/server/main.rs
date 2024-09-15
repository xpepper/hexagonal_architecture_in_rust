use hexarch::config::Config;
use hexarch::domain::blog::service::Service;
use hexarch::inbound::http::{HttpServer, HttpServerConfig};
use hexarch::outbound::author_notifier_using_email_client::AuthorNotifierUsingEmailClient;
use hexarch::outbound::blog_metrics_using_prometheus::BlogMetricsUsingPrometheus;
use hexarch::outbound::blog_repository_using_sqlite::BlogRepositoryUsingSqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let blog_repository = BlogRepositoryUsingSqlite::new(&config.database_url).await?;
    let blog_metrics = BlogMetricsUsingPrometheus::new();
    let author_notifier = AuthorNotifierUsingEmailClient::new();
    let blog_service = Service::new(blog_repository, blog_metrics, author_notifier);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(blog_service, server_config).await?;
    http_server.run().await
}
