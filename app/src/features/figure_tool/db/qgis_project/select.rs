use sqlx::PgConnection;
use sqlx::types::Json;

use crate::{
    app::features::figure_tool::dtos::figure::QgisProjectName,
    qgis::project::{QgisProject, QgisProjectMetadata},
    repo::Select,
};

impl Select<&mut PgConnection, QgisProjectName> for QgisProject {
    async fn select(
        executor: &mut PgConnection,
        name: &QgisProjectName,
    ) -> Result<Self, crate::repo::RepositoryError> {
        let res = sqlx::query_as!(
            QgisProject,
            r#"SELECT name, metadata as "metadata: Json<QgisProjectMetadata>", content, figure_id, low_res FROM qgis.qgis_projects WHERE name = $1"#,
            name.0
        ).fetch_one(executor).await?;

        Ok(res)
    }
}
