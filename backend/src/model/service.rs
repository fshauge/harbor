use super::Application;
use chrono::NaiveDateTime;
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
}

impl Service {
    pub async fn application(&self, pool: &PgPool) -> Result<Application, Error> {
        Application::by_id(self.application_id, pool).await
    }
}
