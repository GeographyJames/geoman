use domain::UserId;
use sqlx::PgConnection;

use crate::{
    features::figure_tool::{dtos::FigureLayerInputDTO, ids::FigureId},
    repo::RepositoryError,
};

pub async fn insert_figure_layers(
    figure_id: &FigureId,
    user_id: &UserId,
    layers: &[FigureLayerInputDTO],
    conn: &mut PgConnection,
) -> Result<(), RepositoryError> {
    for (i, layer) in layers.iter().enumerate() {
        sqlx::query!(
            r#"
INSERT INTO app.figure_layers(
                figure_id,

                name,
                layer_order,

                project_layer_source,
                turbine_layout_id,
                site_boundary_id,
                added_by,
                style_id,
                properties
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            figure_id.as_ref(),
            layer.name.as_ref(),
            i as i32,
            layer
                .source
                .project_layer_src()
                .map(|src| serde_json::json!(src)),
            layer.source.turbine_layout_id().map(|id| id.0),
            layer.source.site_boundary_id().map(|id| id.0),
            user_id.0,
            layer.style_id.as_ref().map(|id| id.as_ref()),
            serde_json::json!(layer.properties)
        )
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}
