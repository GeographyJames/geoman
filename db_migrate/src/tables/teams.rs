use ::chrono::{DateTime, Utc};
use anyhow::Context;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Team {
    id: i32,
    name: String,
    added: DateTime<Utc>,
}

impl Team {
    pub async fn select_all<'a, E>(executor: E) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as("SELECT id, name, added FROM app.teams")
            .fetch_all(executor)
            .await
            .context("failed to query source teams")
    }

    pub async fn migrate<'a, E>(self, executor: E) -> Result<(), anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>,
    {
        {
            sqlx::query!(
            "INSERT INTO app.teams (id, name, added) OVERRIDING SYSTEM VALUE VALUES ($1, $2, $3)",
            self.id,
            self.name,
            self.added
        )
        .execute(executor)
        .await
        .context("failed to insert team")?;

            Ok(())
        }
    }
}
