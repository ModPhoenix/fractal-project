pub mod data;

use axum::{http::StatusCode, routing::get, serve::Serve, Router};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Server = Serve<Router<()>, Router<()>>;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_jwt=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/health_check", get(health_check));

    tracing::debug!("listening on {}", listener.local_addr()?);

    let server = axum::serve(listener, app);

    Ok(server)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
