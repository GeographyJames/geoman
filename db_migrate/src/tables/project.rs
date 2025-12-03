use std::str::FromStr;

use anyhow::Context;
use chrono::{DateTime, Utc};
use domain::enums::{Status, Visibility};
use sqlx::{Acquire, Postgres, prelude::FromRow};

use crate::types::Subdivision;

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

impl Project {
    pub async fn select_all<'a, A>(conn: A) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await.unwrap();
        sqlx::query_as(
            r#"
         SELECT id,
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
        .fetch_all(&mut *executor)
        .await
        .context("failed to select projects")
    }

    pub async fn migrate<'a, A>(self, conn: A) -> Result<(), anyhow::Error>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut conn = conn.acquire().await.unwrap();
        sqlx::query!(
            r#"
        INSERT INTO app.projects (
        id,
        search_area_id,
        search_site_name,
        name,
        slug,
        status,
        visibility,
        country_code,
        owner,
        added_by,
        added,
        last_updated_by,
        last_updated,
        crs_srid,
        team_id
        ) OVERRIDING SYSTEM VALUE VALUES ($1, $2, $3, $4, $5, $6, $7, 'GB', $8, $9, $10,
        (SELECT id FROM app.users WHERE username = 'root-user'), NOW()
        , 27700, $11)
        "#,
            self.id,
            self.search_area_id,
            self.codename,
            self.name,
            self.slug,
            Status::from_str(&self.status.to_string()).context("failed to convert status")?
                as Status,
            if self.private {
                Visibility::Team
            } else {
                Visibility::Public
            } as Visibility,
            self.user_id,
            self.user_id,
            self.added,
            self.team_id
        )
        .execute(&mut *conn)
        .await?;
        let subdivision = Subdivision::from(&self.country);
        sqlx::query!(
            "INSERT INTO app.project_subdivisions (project_id, subdivision_id) VALUES ($1, (SELECT id FROM app.subdivisions WHERE subdivision_code = $2))",
            self.id,
            subdivision.0
        )
        .execute(&mut *conn).await.context(format!("failed to insert subdivision: {}", subdivision.0))?;

        if self.onshore_wind {
            project_technology_insert(self.id, "onshore wind", &mut *conn).await?;
        }
        if self.battery_storage {
            project_technology_insert(self.id, "battery", &mut *conn).await?;
        }
        if self.hydrogen {
            project_technology_insert(self.id, "hydrogen", &mut *conn).await?;
        }
        if self.solar {
            project_technology_insert(self.id, "solar", &mut *conn).await?;
        }
        Ok(())
    }
}

async fn project_technology_insert<'a, E>(
    id: i32,
    technology: &str,
    tx: E,
) -> Result<(), anyhow::Error>
where
    E: sqlx::PgExecutor<'a>,
{
    sqlx::query!(
                "INSERT INTO app.project_technologies (project_id, technology_id) VALUES ($1, (SELECT id FROM app.technologies WHERE name = $2))",
                id,
                technology
            ).execute( tx).await.context(format!("failed to insert technology: {}", technology))?;
    Ok(())
}
