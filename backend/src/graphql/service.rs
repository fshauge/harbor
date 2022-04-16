use crate::model::{Application, NewService, Service};
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

#[derive(Default)]
pub struct ServiceQuery;

#[Object]
impl ServiceQuery {
    async fn services(&self, ctx: &Context<'_>) -> Result<Vec<Service>> {
        let pool = ctx.data::<PgPool>()?;
        let services = Service::all(pool).await?;
        Ok(services)
    }

    async fn service(&self, ctx: &Context<'_>, id: i32) -> Result<Service> {
        let pool = ctx.data::<PgPool>()?;
        let service = Service::by_id(id, pool).await?;
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
        path: String,
        container_id: Option<String>,
    ) -> Result<Service> {
        let pool = ctx.data::<PgPool>()?;
        let service = Service::insert(
            NewService {
                application_id,
                name,
                path,
                container_id,
            },
            pool,
        )
        .await?;
        Ok(service)
    }
}

#[Object]
impl Service {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn path(&self) -> &str {
        &self.path
    }

    async fn container_id(&self) -> Option<&str> {
        self.container_id.as_deref()
    }

    async fn application(&self, ctx: &Context<'_>) -> Result<Application> {
        let pool = ctx.data::<PgPool>()?;
        let application = Application::by_id(self.application_id, pool).await?;
        Ok(application)
    }
}
