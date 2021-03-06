use super::Application;
use anyhow::{bail, Result};
use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions},
    image::BuildImageOptions,
    Docker,
};
use chrono::NaiveDateTime;
use futures::StreamExt;
use sqlx::{Error, FromRow, PgPool};
use std::collections::HashMap;

#[derive(FromRow)]
pub struct Service {
    pub id: i32,
    pub application_id: i32,
    pub name: String,
    pub image: String,
    pub build_context: String,
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

        let slug = self.slug(&application);

        let options = BuildImageOptions {
            remote,
            t: slug,
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
        let application = self.application(pool).await?;
        let slug = self.slug(&application);

        if self.state(&slug, docker).await?.is_some() {
            bail!("service is running")
        }

        let options = CreateContainerOptions { name: slug.clone() };

        let config = Config {
            image: Some(slug.clone()),
            ..Default::default()
        };

        docker.create_container(Some(options), config).await?;
        docker.start_container::<String>(&slug, None).await?;
        Ok(slug)
    }

    pub async fn stop(&self, pool: &PgPool, docker: &Docker) -> Result<String> {
        let application = self.application(pool).await?;
        let slug = self.slug(&application);

        if self.state(&slug, docker).await?.is_none() {
            bail!("service is not running")
        }

        docker.remove_container(&slug, None).await?;
        Ok(slug)
    }

    pub fn slug(&self, application: &Application) -> String {
        format!(
            "{}-{}",
            application.name.to_lowercase().replace(' ', "-"),
            self.image
        )
    }

    pub async fn state(&self, slug: &str, docker: &Docker) -> Result<Option<String>> {
        let options = ListContainersOptions {
            all: true,
            filters: HashMap::from([("name", vec![slug])]),
            ..Default::default()
        };

        let state = docker
            .list_containers(Some(options))
            .await?
            .into_iter()
            .next()
            .and_then(|c| c.state);

        Ok(state)
    }
}
