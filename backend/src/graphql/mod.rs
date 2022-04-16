mod objects;
mod query;

use self::query::Query;
use async_graphql::{EmptyMutation, EmptySubscription};

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, EmptyMutation, EmptySubscription>;

pub fn build() -> SchemaBuilder {
    Schema::build(Query, EmptyMutation, EmptySubscription)
}
