use super::Application;
use anyhow::{bail, Result};
use bollard::{container::Config, image::BuildImageOptions, Docker};
use chrono::NaiveDateTime;
use futures::StreamExt;
use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow)]
pub struct Service {
    pub id: i32,
    pub application_id: i32,
    pub name: String,
    pub image: String,
    pub build_context: String,
    pub container_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewService {
    pub application_id: i32,
    pub name: String,
    pub image: String,
    pub build_context: String,
}

impl Service {
    pub async fn all(pool: &PgPool) -> Result<Vec<Service>, Error> {
        sqlx::query_as!(Service, "SELECT * FROM services")
            .fetch_all(pool)
            .await
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Service, Error> {
        sqlx::query_as!(Service, "SELECT * FROM services WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    pub async fn by_application_id(
        application_id: i32,
        pool: &PgPool,
    ) -> Result<Vec<Service>, Error> {
        sqlx::query_as!(
            Service,
            "SELECT * FROM services WHERE application_id = $1",
            application_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn insert(service: NewService, pool: &PgPool) -> Result<Service, Error> {
        sqlx::query_as!(
            Service,
            r"
            INSERT INTO services (application_id, name, image, build_context)
            VALUES ($1, $2, $3, $4) RETURNING *
            ",
            service.application_id,
            service.name,
            service.image,
            service.build_context
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<Service, Error> {
        sqlx::query_as!(
            Service,
            "DELETE FROM services WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_container_id(
        id: i32,
        container_id: Option<&str>,
        pool: &PgPool,
    ) -> Result<Service, Error> {
        sqlx::query_as!(
            Service,
            r"
            UPDATE services
            SET container_id = $2
            WHERE id = $1
            RETURNING *
            ",
            id,
            container_id
        )
        .fetch_one(pool)
        .await
    }
}

impl Service {
    pub async fn application(&self, pool: &PgPool) -> Result<Application, Error> {
        Application::by_id(self.application_id, pool).await
    }

    pub async fn build(&self, pool: &PgPool, docker: &Docker) -> Result<String> {
        let application = self.application(pool).await?;

        let remote = format!(
            "{}#{}:{}",
            application.repository, application.branch, self.build_context
        );

        let t = format!("{}-{}", application.name, self.image);

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

    pub async fn start(&self, pool: &PgPool, docker: &Docker) -> Result<String> {
        if self.container_id.is_some() {
            bail!("service is running");
        }

        let application = self.application(pool).await?;
        let t = format!("{}-{}", application.name, self.image);

        let config = Config {
            image: Some(t),
            ..Default::default()
        };

        let container_id = docker.create_container::<String, _>(None, config).await?.id;

        docker
            .start_container::<String>(&container_id, None)
            .await?;

        Service::update_container_id(self.id, Some(&container_id), pool).await?;
        Ok(container_id)
    }

    pub async fn stop(&self, pool: &PgPool, docker: &Docker) -> Result<String> {
        let container_id = match self.container_id.clone() {
            Some(container_id) => container_id,
            None => bail!("service is not running"),
        };

        docker.remove_container(&container_id, None).await?;
        Service::update_container_id(self.id, None, pool).await?;
        Ok(container_id)
    }
}
