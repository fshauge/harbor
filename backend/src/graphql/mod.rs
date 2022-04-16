mod application;
mod service;

use self::{application::*, service::*};
use async_graphql::{EmptySubscription, MergedObject};

#[derive(MergedObject, Default)]
pub struct Query(ApplicationQuery, ServiceQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ApplicationMutation, ServiceMutation);

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription>;

pub fn build() -> SchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
