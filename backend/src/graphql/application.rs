use super::service::Service;
use crate::model;
use async_graphql::{Context, Object, Result};
use chrono::NaiveDateTime;
use sqlx::PgPool;

#[derive(Default)]
pub struct ApplicationQuery;

#[Object]
impl ApplicationQuery {
    async fn applications(&self, ctx: &Context<'_>) -> Result<Vec<Application>> {
        let pool = ctx.data::<PgPool>()?;
        let applications = model::Application::all(pool)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(applications)
    }

    async fn application(&self, ctx: &Context<'_>, id: i32) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = model::Application::by_id(id, pool).await?.into();
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
        branch: String,
    ) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = model::Application::insert(
            model::NewApplication {
                name,
                repository,
                branch,
            },
            pool,
        )
        .await?
        .into();
        Ok(application)
    }

    async fn delete_application(&self, ctx: &Context<'_>, id: i32) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = model::Application::delete(id, pool).await?.into();
        Ok(application)
    }
}

pub struct Application(pub model::Application);

impl From<model::Application> for Application {
    fn from(application: model::Application) -> Self {
        Self(application)
    }
}

#[Object]
impl Application {
    async fn id(&self) -> i32 {
        self.0.id
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn repository(&self) -> &str {
        &self.0.repository
    }

    async fn branch(&self) -> &str {
        &self.0.branch
    }

    async fn created_at(&self) -> NaiveDateTime {
        self.0.created_at
    }

    async fn updated_at(&self) -> NaiveDateTime {
        self.0.updated_at
    }

    async fn services(&self, ctx: &Context<'_>) -> Result<Vec<Service>> {
        let pool = ctx.data::<PgPool>()?;
        let services = model::Service::by_application_id(self.0.id, pool)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(services)
    }
}
