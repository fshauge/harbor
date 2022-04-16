use chrono::NaiveDateTime;
use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow)]
pub struct Application {
    pub id: i32,
    pub name: String,
    pub repository: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewApplication {
    pub name: String,
    pub repository: String,
}

impl Application {
    pub async fn all(pool: &PgPool) -> Result<Vec<Application>, Error> {
        sqlx::query_as!(Application, "SELECT * FROM applications")
            .fetch_all(pool)
            .await
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Application, Error> {
        sqlx::query_as!(Application, "SELECT * FROM applications WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    pub async fn insert(application: NewApplication, pool: &PgPool) -> Result<Application, Error> {
        sqlx::query_as!(
            Application,
            "INSERT INTO applications (name, repository) VALUES ($1, $2) RETURNING *",
            application.name,
            application.repository
        )
        .fetch_one(pool)
        .await
    }
}
