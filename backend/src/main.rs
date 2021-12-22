mod graphql;

use std::net::SocketAddr;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, Html},
    routing::{get, post},
    AddExtensionLayer, Router, Server,
};
use bollard::Docker;

const PATH: &str = "/graphql";

async fn graphql_handler(
    Extension(schema): Extension<graphql::Schema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let request = request.into_inner();
    schema.execute(request).await.into()
}

async fn playground_handler() -> Html<String> {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(PATH)))
}

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let app = Router::new()
        .route(PATH, get(playground_handler))
        .route(PATH, post(graphql_handler))
        .layer(AddExtensionLayer::new(graphql::create_schema(docker)));

    let port = std::env::var("PORT").unwrap().parse().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
