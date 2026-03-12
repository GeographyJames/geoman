use sqlx::PgPool;

use crate::{
    app::features::figure_tool::{
        db::figure_layer::insert_figure_layers,
        dtos::figure::FigureInputDTO,
        enums::FigureStatus,
        ids::FigureId,
    },
    repo::{RepositoryError, Update},
};

impl<'a> Update<&'a PgPool, FigureId> for FigureInputDTO {
    async fn update(
        &self,
        executor: &'a PgPool,
        id: &FigureId,
    ) -> Result<(), crate::repo::RepositoryError> {
        let mut tx = executor.begin().await?;

        sqlx::query!(
            r#"
UPDATE app.figures SET
                project_id = $1,
                qgis_project_uuid = $2,
                last_updated_by = $3,
                page_width_mm = $4,
                page_height_mm = $5,
                margin_mm = $6,
                legend_width_mm = $7,
                scale = $8,
                srid = $9,
                properties = $10,
                main_map_base_map_id = $11,
                overview_map_base_map_id = $12,
                status = $13
                 WHERE app.figures.id = $14

            "#,
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
            self.overvier_map_base_map_id.as_ref().map(|id| id.as_ref()),
            &self.status as &FigureStatus,
            id.as_ref(),
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!(
            "DELETE FROM app.figure_layers WHERE app.figure_layers.figure_id = $1",
            id.as_ref()
        )
        .execute(&mut *tx)
        .await?;
        insert_figure_layers(id, &self.user_id, &self.layers, &mut tx).await?;

        tx.commit().await?;
        Ok(())
    }
}
