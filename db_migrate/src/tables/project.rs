use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::traits::SelectAll;

#[derive(FromRow)]
pub struct Project {
    id: i32,
    search_area_id: Option<i32>,
    user_id: i32,
    name: String,
    slug: String,
    codename: Option<String>,
    country: geoman::domain::enums::Country,
    status: geoman::domain::enums::ProjectStatus,
    added: DateTime<Utc>,
    team_id: i32,
    onshore_wind: bool,
    solar: bool,
    hydrogen: bool,
    battery_storage: bool,
    private: bool,
}

impl SelectAll for Project {
    async fn select_all<'a, E>(executor: E) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(
            r#"
         SELECT id
                search_area_id,
                user_id,
                name,
                slug,
                codename,
                country,
                status,
                added,
                team_id,
                onshore_wind,
                solar,
                hydrogen,
                battery_storage,
                private
           FROM app.projects

        "#,
        )
        .fetch_all(executor)
        .await
        .context("failed to select projects")
    }
}
