use domain::UserId;
use sqlx::{Acquire, Postgres};

use crate::{
    features::figure_tool::{
        db::figure_layer::insert_figure_layers,
        dtos::FigureLayerInputDTO,
        handlers::figure::{FigureLayerPayload, FigureUpdatePayload},
        ids::FigureId,
    },
    repo::{RepositoryError, traits::Update},
};

impl Update for (FigureUpdatePayload, FigureId, UserId) {
    type Id = FigureId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let (payload, id, user_id) = self;
        let mut tx = conn.begin().await?;

        let properties = payload
            .properties
            .as_ref()
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| {
                RepositoryError::UnexpectedError(
                    anyhow::anyhow!("failed to serialize figure properties: {}", e).into(),
                )
            })?;

        let res = sqlx::query!(
            r#"
            UPDATE app.figures
            SET scale                    = COALESCE($1,  scale),
                legend_width_mm          = COALESCE($2,  legend_width_mm),
                margin_mm                = COALESCE($3,  margin_mm),
                page_width_mm            = COALESCE($4,  page_width_mm),
                page_height_mm           = COALESCE($5,  page_height_mm),
                srid                     = COALESCE($6,  srid),
                properties               = COALESCE($7,  properties),
                status                   = COALESCE($8,  status),
                main_map_base_map_id     = CASE WHEN $9  THEN $10 ELSE main_map_base_map_id END,
                overview_map_base_map_id = CASE WHEN $11 THEN $12 ELSE overview_map_base_map_id END,
                qgis_project_uuid        = $13,
                last_updated             = NOW(),
                last_updated_by          = $14
            WHERE id = $15
            RETURNING id
            "#,
            payload.scale.map(|v| v as i32),
            payload.legend_width_mm.map(|v| v as i32),
            payload.margin_mm.map(|v| v as i32),
            payload.page_width_mm.map(|v| v as i32),
            payload.page_height_mm.map(|v| v as i32),
            payload.srid.map(|v| v as i32),
            properties,
            payload.status as _,
            payload.main_map_base_map_id.is_some(),
            payload.main_map_base_map_id.clone().flatten().map(|id| id.0),
            payload.overview_map_base_map_id.is_some(),
            payload.overview_map_base_map_id.clone().flatten().map(|id| id.0),
            uuid::Uuid::new_v4(),
            user_id.0,
            id.0,
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(ref layers) = payload.layers {
            sqlx::query!(
                "DELETE FROM app.figure_layers WHERE figure_id = $1",
                id.0
            )
            .execute(&mut *tx)
            .await?;

            let layer_inputs: Vec<FigureLayerInputDTO> = layers
                .iter()
                .cloned()
                .map(|l: FigureLayerPayload| l.try_into())
                .collect::<Result<_, String>>()
                .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e).into()))?;

            insert_figure_layers(id, user_id, &layer_inputs, &mut *tx).await?;
        }

        tx.commit().await?;
        Ok(FigureId(res.id))
    }
}
