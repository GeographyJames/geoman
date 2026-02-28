use domain::{LayoutId, ProjectId, UserId, name::NameInputDTO};
use sqlx::PgPool;

use crate::repo::RepositoryError;

pub async fn duplicate_turbine_layout(
    pool: &PgPool,
    project_id: ProjectId,
    source_layout_id: LayoutId,
    user_id: UserId,
    name: Option<NameInputDTO>,
    hub_height_mm: Option<i32>,
    rotor_diameter_mm: Option<i32>,
    primary: Option<bool>,
) -> Result<LayoutId, RepositoryError> {
    let mut tx = pool.begin().await?;

    if primary == Some(true) {
        sqlx::query!(
            "UPDATE app.turbine_layouts SET is_primary=false WHERE is_primary=true AND project_id=$1",
            project_id.0
        )
        .execute(&mut *tx)
        .await?;
    }

    // INSERT...SELECT so we can COALESCE the name from the source and detect a missing
    // source layout in a single query (fetch_optional returns None if source not found).
    let new_id = sqlx::query_scalar!(
        "INSERT INTO app.turbine_layouts(project_id, name, is_primary, added_by, last_updated_by)
         SELECT $1, COALESCE($2, tl.name), COALESCE($3, false), $4, $4
         FROM app.turbine_layouts tl
         WHERE tl.id = $5
           AND tl.project_id = $1
         RETURNING id",
        project_id.0,
        name.as_ref().map(|n| n.as_ref()),
        primary,
        user_id.0,
        source_layout_id.0,
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(RepositoryError::RowNotFound)?;

    sqlx::query!(
        "INSERT INTO app.turbines(layout_id, turbine_number, hub_height_mm, rotor_diameter_mm, geom)
         SELECT $1, t.turbine_number, COALESCE($2, t.hub_height_mm), COALESCE($3, t.rotor_diameter_mm), t.geom
         FROM app.turbines t
         WHERE t.layout_id = $4",
        new_id,
        hub_height_mm,
        rotor_diameter_mm,
        source_layout_id.0,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(LayoutId(new_id))
}
