use sqlx::PgConnection;

use crate::{
    app::features::figure_tool::{
        dtos::figure_layer::FigureLayerInputDTO,
        ids::FigureId,
    },
    domain::dtos::UserId,
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
                user_id,
                name,
                layer_order,

                project_layer_source,
                turbine_layout_id,
                site_boundary_id,
                added_by,
                style_id,
                properties
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            figure_id.as_ref(),
            user_id.as_ref(),
            layer.name.as_ref(),
            i as i32,
            layer
                .source
                .project_layer_src()
                .map(|src| serde_json::json!(src)),
            layer.source.turbine_layout_id().map(|id| id.0),
            layer.source.site_boundary_id().map(|id| id.0),
            user_id.as_ref(),
            layer.style_id.as_ref().map(|id| id.as_ref()),
            serde_json::json!(layer.properties)
        )
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}
