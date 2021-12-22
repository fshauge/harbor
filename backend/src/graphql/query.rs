use std::sync::Arc;

use super::objects::Docker;
use async_graphql::{Context, Object, Result};

pub struct Query;

#[Object]
impl Query {
    async fn docker(&self, ctx: &Context<'_>) -> Result<Docker> {
        let docker: &Arc<bollard::Docker> = ctx.data()?;
        Ok(Docker(docker.clone()))
    }
}
