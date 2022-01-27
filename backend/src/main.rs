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
use env_logger::Env;

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
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let docker = Docker::connect_with_socket_defaults()?;
    let schema = graphql::build_schema().data(Arc::new(docker)).finish();

    let app = Router::new()
        .route(PATH, get(playground_handler))
        .route(PATH, post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    let port = std::env::var("PORT")?.parse()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
