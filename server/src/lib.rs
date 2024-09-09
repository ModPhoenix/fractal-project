use kuzu::Database;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub mod data;
pub mod graphql;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    http::Method,
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
    .finish();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/health_check", get(health_check))
        .layer(cors)
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
