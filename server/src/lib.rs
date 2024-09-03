use kuzu::Database;
use std::sync::Arc;

pub mod data;
pub mod graphql;

use async_graphql::{extensions::ApolloTracing, http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    routing::get,
    serve::Serve,
    Router,
};
use graphql::{MutationRoot, QueryRoot};
use tokio::net::TcpListener;

pub type Server = Serve<Router<()>, Router<()>>;

pub fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let state = Arc::new(db);

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(state.clone())
    .extension(ApolloTracing)
    .finish();

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/health_check", get(health_check))
        .with_state(state);

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
