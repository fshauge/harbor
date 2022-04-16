mod application;

use self::application::*;
use async_graphql::{EmptySubscription, MergedObject};

#[derive(MergedObject, Default)]
pub struct Query(ApplicationQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ApplicationMutation);

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription>;

pub fn build() -> SchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
