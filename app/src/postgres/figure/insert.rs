use domain::{FigureId, UserId, figure::FigureInputDTO};
use sqlx::{Acquire, Postgres};

use crate::{postgres::figure_layer::insert_figure_layers, repo::traits::Insert};

impl Insert for (&FigureInputDTO, UserId) {
    type Id = FigureId;

    async fn insert<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut tx = conn.begin().await?;
        let (dto, user_id) = self;

        let figure_id = FigureId(
            sqlx::query_scalar!(
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
                dto.project_id.0,
                dto.qgis_project_uuid,
                user_id.0,
                dto.page_width_mm as i32,
                dto.page_height_mm as i32,
                dto.margin_mm as i32,
                dto.legend_width_mm as i32,
                dto.scale as i32,
                dto.srid as i32,
                serde_json::to_value(&dto.properties).unwrap_or_default(),
                dto.main_map_base_map_id.map(|id| id.0),
                dto.overvier_map_base_map_id.map(|id| id.0)
            )
            .fetch_one(&mut *tx)
            .await?,
        );

        insert_figure_layers(figure_id, *user_id, &dto.layers, &mut tx).await?;
        tx.commit().await?;
        Ok(figure_id)
    }
}
