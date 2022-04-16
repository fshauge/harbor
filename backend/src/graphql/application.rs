use crate::model::{Application, NewApplication, Service};
use async_graphql::{Context, Object, Result};
use chrono::NaiveDateTime;
use sqlx::PgPool;

#[derive(Default)]
pub struct ApplicationQuery;

#[Object]
impl ApplicationQuery {
    async fn applications(&self, ctx: &Context<'_>) -> Result<Vec<Application>> {
        let pool = ctx.data::<PgPool>()?;
        let applications = Application::all(pool).await?;
        Ok(applications)
    }

    async fn application(&self, ctx: &Context<'_>, id: i32) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = Application::by_id(id, pool).await?;
        Ok(application)
    }
}

#[derive(Default)]
pub struct ApplicationMutation;

#[Object]
impl ApplicationMutation {
    async fn insert_application(
        &self,
        ctx: &Context<'_>,
        name: String,
        repository: String,
    ) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = Application::insert(NewApplication { name, repository }, pool).await?;
        Ok(application)
    }
}

#[Object]
impl Application {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn repository(&self) -> &str {
        &self.repository
    }

    async fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    async fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    async fn services(&self, ctx: &Context<'_>) -> Result<Vec<Service>> {
        let pool = ctx.data::<PgPool>()?;
        let services = Service::by_application_id(self.id, pool).await?;
        Ok(services)
    }
}
