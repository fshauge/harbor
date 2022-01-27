mod objects;
mod query;

use self::query::Query;
use async_graphql::{EmptyMutation, EmptySubscription, SchemaBuilder};

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
}
