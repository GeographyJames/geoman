use sqlx::{PgConnection, types::Json};

use crate::{features::figure_tool::dtos::QgisProjectName, repo::RepositoryError};
use qgis::project::{QgisProject, QgisProjectMetadata};

pub async fn select_qgis_project(
    conn: &mut PgConnection,
    name: &QgisProjectName,
) -> Result<QgisProject, RepositoryError> {
    let res = sqlx::query_as!(
        QgisProject,
        r#"SELECT name, metadata as "metadata: Json<QgisProjectMetadata>", content, figure_id, low_res FROM public.qgis_projects WHERE name = $1"#,
        name.0
    )
    .fetch_one(conn)
    .await?;
    Ok(res)
}
