use std::str::FromStr;

use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::types::Subdivision;
use domain::enums::Status;

#[derive(FromRow)]
pub struct SearchArea {
    id: i32,
    name: String,
    user_id: i32,
    slug: String,
    country: geoman::domain::enums::Country,
    status: geoman::domain::enums::SearchAreaStatus,
    added: DateTime<Utc>,
    geom: Vec<u8>,
}

impl SearchArea {
    pub async fn select_all<'a, E>(executor: E) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(
            r#"
         SELECT id,
                name,
                user_id,
                slug,
                country,
                status,
                added,
                ST_AsBinary(geom) as geom
        FROM app.search_areas
        "#,
        )
        .fetch_all(executor)
        .await
        .context("failed to select search areas")
    }

    pub async fn migrate<'a, E>(self, executor: E) -> Result<(), anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>,
    {
        sqlx::query!(
            r#"
        INSERT INTO app.search_areas (
            id,
            name,
            team_id,
            slug,
            country_code,
            subdivision,
            status,
            added,
            added_by,
            last_updated,
            last_updated_by,
            geom
            ) OVERRIDING SYSTEM VALUE VALUES (
             $1,
             $2,
             (SELECT id FROM app.teams WHERE name = 'Onshore Wind and Hydrogen Development'),
              $3,
              'GB',
        (SELECT id FROM app.subdivisions WHERE subdivision_code = $4),
        $5, $6, $7,
         NOW(), (SELECT id FROM app.users WHERE username = 'root-user'), ST_GeomFromWKB($8))
            "#,
            self.id,
            self.name,
            self.slug,
            Subdivision::from(&self.country).0,
            Status::from_str(&self.status.to_string()).context("failed to convert status")?
                as Status,
            self.added,
            self.user_id,
            self.geom
        )
        .execute(executor)
        .await?;
        Ok(())
    }
}
