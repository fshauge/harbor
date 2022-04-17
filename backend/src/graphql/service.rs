use super::application::Application;
use crate::model;
use async_graphql::{Context, Object, Result};
use bollard::{image::BuildImageOptions, Docker};
use futures::StreamExt;
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

    async fn build_service(&self, ctx: &Context<'_>, id: i32) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;
        let service = model::Service::by_id(id, pool).await?;
        let application = service.application(pool).await?;
        let docker = ctx.data::<Docker>()?;

        let remote = format!(
            "{}#{}:{}",
            application.repository, application.branch, service.build_context
        );

        let t = format!("{}-{}", application.name, service.image);

        let options = BuildImageOptions {
            remote,
            t,
            rm: true,
            ..Default::default()
        };

        let build_info = docker
            .build_image(options, None, None)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter_map(|b| b.stream)
            .collect::<String>();

        Ok(build_info)
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

    async fn application(&self, ctx: &Context<'_>) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = self.0.application(pool).await?.into();
        Ok(application)
    }
}
