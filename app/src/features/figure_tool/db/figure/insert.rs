use sqlx::PgPool;

use crate::{
    app::features::figure_tool::{
        db::figure_layer::insert_figure_layers,
        dtos::figure::FigureInputDTO,
        ids::FigureId,
    },
    repo::{Insert, RepositoryError},
};

impl<'a> Insert<&'a PgPool, FigureId> for FigureInputDTO {
    async fn insert(&self, executor: &'a PgPool) -> Result<FigureId, crate::repo::RepositoryError> {
        let mut tx = executor.begin().await?;

        let figure_id = FigureId(sqlx::query_scalar!(
            r#"
INSERT INTO app.figures(
                project_id,
                qgis_project_uuid,
                added_by,
                last_updated_by,
                page_width_mm,
                page_height_mm,
                margin_mm,
                legend_width_mm,
                scale,
                srid,
                properties,
                main_map_base_map_id,
                overview_map_base_map_id
                )
     VALUES ($1, $2, $3, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id"#,
            self.project_id.as_ref(),
            self.qgis_project_uuid,
            self.user_id.as_ref(),
            self.page_width_mm as i32,
            self.page_height_mm as i32,
            self.margin_mm as i32,
            self.legend_width_mm as i32,
            self.scale as i32,
            self.srid as i32,
            serde_json::to_value(&self.properties).map_err(
                |e| RepositoryError::UnexpectedError(
                    anyhow::anyhow!("failed to serialize figure properties: {}", e).into()
                )
            )?,
            self.main_map_base_map_id.as_ref().map(|id| id.as_ref()),
            self.overvier_map_base_map_id.as_ref().map(|id| id.as_ref())
        )
        .fetch_one(&mut *tx)
        .await?);

        insert_figure_layers(&figure_id, &self.user_id, &self.layers, &mut tx).await?;
        tx.commit().await?;
        Ok(figure_id)
    }
}
