pub mod data;
pub mod graphql;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    routing::get,
    serve::Serve,
    Router,
};
use graphql::QueryRoot;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Server = Serve<Router<()>, Router<()>>;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/health_check", get(health_check));

    tracing::debug!(
        "GraphiQL IDE: http://{}:{}",
        listener.local_addr()?.ip(),
        listener.local_addr()?.port()
    );

    let server = axum::serve(listener, app);

    Ok(server)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}
