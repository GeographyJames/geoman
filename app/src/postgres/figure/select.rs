use chrono::Utc;
use sqlx::{FromRow, PgConnection, PgPool};

use domain::{
    DataProviderLayerId, FigureId, ProjectId, UserId,
    enums::Status,
    figure::{FigureOutputDTO, FigureProperties},
};

use crate::{postgres::figure_layer::select_all_layers_for_figure, repo::RepositoryError};

#[derive(FromRow)]
struct FigureSelection {
    id: FigureId,
    project_id: ProjectId,
    project_name: String,
    main_map_base_map_id: Option<DataProviderLayerId>,
    overview_map_base_map_id: Option<DataProviderLayerId>,
    qgis_project_uuid: uuid::Uuid,
    added_by: UserId,
    added_by_first_name: String,
    added_by_last_name: String,
    last_updated_by: UserId,
    last_updated_by_first_name: String,
    last_updated_by_last_name: String,
    status: Status,
    added: chrono::DateTime<Utc>,
    last_updated: chrono::DateTime<Utc>,
    page_width_mm: i32,
    page_height_mm: i32,
    margin_mm: i32,
    legend_width_mm: i32,
    scale: i32,
    srid: i32,
    properties: sqlx::types::Json<FigureProperties>,
}

const BASE_QUERY: &str = r#"
SELECT  f.id,
        f.project_id,
        p.name as project_name,
        f.added_by,
        f.qgis_project_uuid,
        f.added,
        u1.first_name as added_by_first_name,
        u1.last_name as added_by_last_name,
        f.last_updated_by,
        f.last_updated,
        u2.first_name as last_updated_by_first_name,
        u2.last_name as last_updated_by_last_name,
        f.status,
        f.page_width_mm,
        f.page_height_mm,
        f.scale,
        f.margin_mm,
        f.legend_width_mm,
        f.srid,
        f.properties,
        f.main_map_base_map_id,
        f.overview_map_base_map_id
FROM    app.figures f
JOIN    app.users u1 ON u1.id = f.added_by
JOIN    app.users u2 ON u2.id = f.last_updated_by
JOIN    app.projects p ON p.id = f.project_id
"#;

fn from_selection(
    row: FigureSelection,
    layers: Vec<domain::figure_layer::FigureLayerOutputDTO>,
) -> FigureOutputDTO {
    FigureOutputDTO {
        id: row.id,
        project_id: row.project_id,
        project_name: row.project_name,
        main_map_base_map_id: row.main_map_base_map_id,
        overview_map_base_map_id: row.overview_map_base_map_id,
        qgis_project_uuid: row.qgis_project_uuid,
        added_by: row.added_by,
        added_by_first_name: row.added_by_first_name,
        added_by_last_name: row.added_by_last_name,
        last_updated_by: row.last_updated_by,
        last_updated_by_first_name: row.last_updated_by_first_name,
        last_updated_by_last_name: row.last_updated_by_last_name,
        status: row.status,
        added: row.added,
        last_updated: row.last_updated,
        page_width_mm: row.page_width_mm,
        page_height_mm: row.page_height_mm,
        margin_mm: row.margin_mm,
        legend_width_mm: row.legend_width_mm,
        scale: row.scale,
        srid: row.srid,
        properties: row.properties.0,
        layers: Some(layers),
    }
}

pub async fn select_figures_for_project(
    pool: &PgPool,
    project_id: ProjectId,
) -> Result<Vec<FigureOutputDTO>, RepositoryError> {
    let mut tx = pool.begin().await?;
    let rows: Vec<FigureSelection> = sqlx::query_as(&format!(
        "{} WHERE f.project_id = $1 AND f.status != 'DELETED'",
        BASE_QUERY
    ))
    .bind(project_id.0)
    .fetch_all(&mut *tx)
    .await?;
    let mut figures = Vec::new();
    for row in rows {
        let layers = select_all_layers_for_figure(&mut tx, row.id).await?;
        figures.push(from_selection(row, layers));
    }
    Ok(figures)
}

pub async fn select_figure(
    pool: &PgPool,
    figure_id: FigureId,
) -> Result<FigureOutputDTO, RepositoryError> {
    let mut tx = pool.begin().await?;
    let row: FigureSelection =
        sqlx::query_as(&format!("{} WHERE f.id = $1", BASE_QUERY))
            .bind(figure_id.0)
            .fetch_one(&mut *tx)
            .await
            .map_err(RepositoryError::from)?;
    let layers = select_all_layers_for_figure(&mut tx, row.id).await?;
    Ok(from_selection(row, layers))
}

pub async fn select_figure_with_conn(
    conn: &mut PgConnection,
    figure_id: FigureId,
) -> Result<FigureOutputDTO, RepositoryError> {
    let row: FigureSelection = sqlx::query_as(&format!("{} WHERE f.id = $1", BASE_QUERY))
        .bind(figure_id.0)
        .fetch_one(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;
    let layers = select_all_layers_for_figure(&mut *conn, row.id).await?;
    Ok(from_selection(row, layers))
}
