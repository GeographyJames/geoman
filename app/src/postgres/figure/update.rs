use domain::{FigureId, UserId, enums::Status, figure::FigureInputDTO};
use sqlx::{Acquire, Postgres};

use crate::{postgres::figure_layer::insert_figure_layers, repo::{RepositoryError, traits::Update}};

impl Update for (&FigureInputDTO, FigureId, UserId) {
    type Id = FigureId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut tx = conn.begin().await?;
        let (dto, figure_id, user_id) = self;

        sqlx::query!(
            r#"
UPDATE app.figures SET
    project_id = $1,
    qgis_project_uuid = $2,
    last_updated_by = $3,
    last_updated = NOW(),
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
WHERE id = $14
            "#,
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
            dto.overvier_map_base_map_id.map(|id| id.0),
            &dto.status as &Status,
            figure_id.0,
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "DELETE FROM app.figure_layers WHERE figure_id = $1",
            figure_id.0
        )
        .execute(&mut *tx)
        .await?;

        insert_figure_layers(*figure_id, *user_id, &dto.layers, &mut tx).await?;

        tx.commit().await?;
        Ok(*figure_id)
    }
}
