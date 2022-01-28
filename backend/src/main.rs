mod graphql;

use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::Html,
    routing::{get, post},
    AddExtensionLayer, Router, Server,
};
use bollard::Docker;
use sea_orm::Database;

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
    tracing_subscriber::fmt::init();

    if dotenv::dotenv().is_ok() {
        log::info!("Loaded environment variables from file");
    }

    let docker = Docker::connect_with_socket_defaults()?;
    let database_url = std::env::var("DATABASE_URL")?;
    let database = Database::connect(database_url).await?;

    let schema = graphql::build_schema()
        .data(Arc::new(docker))
        .data(Arc::new(database))
        .finish();

    let app = Router::new()
        .route(PATH, get(playground_handler))
        .route(PATH, post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    let port = std::env::var("PORT")?.parse()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
