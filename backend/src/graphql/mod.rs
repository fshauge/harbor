mod objects;
mod query;

use std::sync::Arc;

use self::query::Query;
use async_graphql::{EmptyMutation, EmptySubscription};
use bollard::Docker;

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema(docker: Docker) -> Schema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(Arc::new(docker))
        .finish()
}
