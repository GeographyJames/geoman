use qgis::project::QgisProject;
use sqlx::PgPool;

use crate::repo::RepositoryError;

/// Insert a QGIS project into the database.
///
/// Matches prototype behaviour: wraps in a transaction that first deletes any existing
/// low-res projects for the same figure (so an old low-res project is cleaned up whenever a
/// new project — PDF or JPG — is stored), then inserts the new record.
pub async fn insert_qgis_project(
    pool: &PgPool,
    project: &QgisProject,
) -> Result<(), RepositoryError> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "DELETE FROM public.qgis_projects WHERE figure_id = $1 AND low_res = true",
        project.figure_id,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"INSERT INTO public.qgis_projects (name, metadata, content, figure_id, low_res)
           VALUES ($1, $2, $3, $4, $5)"#,
        project.name,
        project.metadata as _,
        project.content,
        project.figure_id,
        project.low_res,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Returns true if a project with this name already exists. Used by the handler to decide
/// whether to regenerate a project or reuse the cached one.
pub async fn qgis_project_exists(pool: &PgPool, name: &str) -> Result<bool, RepositoryError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM public.qgis_projects WHERE name = $1)",
        name
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(false);
    Ok(exists)
}

/// Returns the raw `.qgz` bytes for a project by name, or `None` if not found.
pub async fn get_qgis_project_content(
    pool: &PgPool,
    name: &str,
) -> Result<Option<Vec<u8>>, RepositoryError> {
    let content = sqlx::query_scalar!(
        "SELECT content FROM public.qgis_projects WHERE name = $1",
        name
    )
    .fetch_optional(pool)
    .await?;
    Ok(content)
}
