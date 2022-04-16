use chrono::NaiveDateTime;
use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow)]
pub struct Service {
    pub id: i32,
    pub application_id: i32,
    pub name: String,
    pub path: String,
    pub container_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewService {
    pub application_id: i32,
    pub name: String,
    pub path: String,
    pub container_id: Option<String>,
}

impl Service {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(Self, "SELECT * FROM services")
            .fetch_all(pool)
            .await
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self, Error> {
        sqlx::query_as!(Self, "SELECT * FROM services WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    pub async fn by_application_id(application_id: i32, pool: &PgPool) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM services WHERE application_id = $1",
            application_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn insert(service: NewService, pool: &PgPool) -> Result<Self, Error> {
        sqlx::query_as!(
            Self,
            r#"
                INSERT INTO services (application_id, name, path, container_id)
                VALUES ($1, $2, $3, $4) RETURNING *
            "#,
            service.application_id,
            service.name,
            service.path,
            service.container_id
        )
        .fetch_one(pool)
        .await
    }
}
