use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::traits::{Migrate, SelectAll};

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub admin: bool,
    pub team_id: i32,
    pub added: DateTime<Utc>,
}

impl SelectAll for User {
    async fn select_all<'a, E>(executor: E) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(
            "SELECT id, username, first_name, last_name, admin, added, team_id FROM app.users",
        )
        .fetch_all(executor)
        .await
        .context("failed to query source users")
    }
}

impl Migrate for User {
    async fn migrate<'a, E>(self, executor: E) -> Result<(), anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>,
    {
        sqlx::query!(
            "INSERT INTO app.users (id, username, first_name, last_name, admin, team_id, added) OVERRIDING SYSTEM VALUE VALUES ($1, $2, $3, $4, $5, $6, $7)",
            self.id,
            self.username,
            self.first_name,
            self.last_name,
            self.admin,
            self.team_id,
            self.added
        ).execute(executor).await.context("failed to insert user")?;
        Ok(())
    }
}
