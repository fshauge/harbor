mod graphql;

use anyhow::Result;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::Html,
    routing::{get, post},
    Router, Server,
};
use bollard::Docker;
use serde::Deserialize;
use sqlx::PgPool;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct Config {
    port: Option<u16>,
    database_url: String,
}

const PATH: &str = "/graphql";

async fn graphql_handler(
    Extension(schema): Extension<graphql::Schema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let request = request.into_inner();
    schema.execute(request).await.into()
}

async fn playground_handler() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new(PATH)))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = envy::from_env::<Config>()?;
    let pool = PgPool::connect(&config.database_url).await?;
    let docker = Docker::connect_with_socket_defaults()?;
    let schema = graphql::build().data(pool).data(docker).finish();

    let app = Router::new()
        .route(PATH, get(playground_handler))
        .route(PATH, post(graphql_handler))
        .layer(Extension(schema));

    let port = config.port.unwrap_or(4000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Launched on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
