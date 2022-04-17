use super::application::Application;
use crate::model;
use async_graphql::{Context, Object, Result};
use bollard::Docker;
use sqlx::PgPool;

#[derive(Default)]
pub struct ServiceQuery;

#[Object]
impl ServiceQuery {
    async fn services(&self, ctx: &Context<'_>) -> Result<Vec<Service>> {
        let pool = ctx.data::<PgPool>()?;
        let services = model::Service::all(pool)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(services)
    }

    async fn service(&self, ctx: &Context<'_>, id: i32) -> Result<Service> {
        let pool = ctx.data::<PgPool>()?;
        let service = model::Service::by_id(id, pool).await?.into();
        Ok(service)
    }
}

#[derive(Default)]
pub struct ServiceMutation;

#[Object]
impl ServiceMutation {
    async fn insert_service(
        &self,
        ctx: &Context<'_>,
        application_id: i32,
        name: String,
        image: String,
        build_context: String,
    ) -> Result<Service> {
        let pool = ctx.data::<PgPool>()?;
        let service = model::Service::insert(
            model::NewService {
                application_id,
                name,
                image,
                build_context,
            },
            pool,
        )
        .await?
        .into();
        Ok(service)
    }

    async fn delete_service(&self, ctx: &Context<'_>, id: i32) -> Result<Service> {
        let pool = ctx.data::<PgPool>()?;
        let service = model::Service::delete(id, pool).await?.into();
        Ok(service)
    }

    async fn build_service(&self, ctx: &Context<'_>, id: i32) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;
        let service = model::Service::by_id(id, pool).await?;
        let docker = ctx.data::<Docker>()?;
        let build_info = service.build(pool, docker).await?;
        Ok(build_info)
    }

    async fn start_service(&self, ctx: &Context<'_>, id: i32) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;
        let service = model::Service::by_id(id, pool).await?;
        let docker = ctx.data::<Docker>()?;
        let container_id = service.start(pool, docker).await?;
        Ok(container_id)
    }

    async fn stop_service(&self, ctx: &Context<'_>, id: i32) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;
        let service = model::Service::by_id(id, pool).await?;
        let docker = ctx.data::<Docker>()?;
        let container_id = service.stop(pool, docker).await?;
        Ok(container_id)
    }
}

pub struct Service(pub model::Service);

impl From<model::Service> for Service {
    fn from(service: model::Service) -> Self {
        Self(service)
    }
}

#[Object]
impl Service {
    async fn id(&self) -> i32 {
        self.0.id
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn image(&self) -> &str {
        &self.0.image
    }

    async fn build_context(&self) -> &str {
        &self.0.build_context
    }

    async fn container_id(&self) -> Option<&str> {
        self.0.container_id.as_deref()
    }

    async fn slug(&self, ctx: &Context<'_>) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;
        let application = self.0.application(pool).await?;
        let slug = self.0.slug(&application);
        Ok(slug)
    }

    async fn application(&self, ctx: &Context<'_>) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = self.0.application(pool).await?.into();
        Ok(application)
    }
}
