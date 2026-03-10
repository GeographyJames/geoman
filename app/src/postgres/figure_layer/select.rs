use std::str::FromStr;

use chrono::Utc;
use serde::Deserialize;
use sqlx::{FromRow, PgConnection};

use domain::{
    FeatureId, FigureId, FigureLayerId, LayoutId, StyleId, UserId,
    bounding_box::BoundingBox,
    figure_layer::{
        FigureLayerDatasourceOutput, FigureLayerOutputDTO, LayerProperties, ProjectLayer,
        SiteAssetDatasourceOutputDTO, SiteAssetId,
    },
    pg_table::{PgTableInvalidOutputDTO, PgTableOutputDTO, WkbType},
};

use crate::repo::RepositoryError;

#[derive(FromRow, Deserialize)]
struct FigureLayerSelection {
    id: FigureLayerId,
    style_id: Option<StyleId>,
    figure_id: FigureId,
    name: String,
    properties: sqlx::types::Json<LayerProperties>,

    project_layer_source: Option<sqlx::types::Json<PgTableSelection>>,
    turbine_layout_id: Option<LayoutId>,
    site_boundary_id: Option<FeatureId>,
    layout_name: Option<String>,
    site_boundary_name: Option<String>,

    user_id: UserId,
    added_by_first_name: String,
    added_by_last_name: String,
    added: chrono::DateTime<Utc>,
    styleqml: Option<String>,
}

#[derive(Deserialize)]
struct PgTableSelection {
    table: String,
    schema: String,
}

async fn select_project_layer_datasource(
    pg_table: PgTableSelection,
    conn: &mut PgConnection,
) -> Result<ProjectLayer, RepositoryError> {
    let PgTableSelection { table, schema } = pg_table;
    let ds = if let Some((geom_type, epsg_id)) = sqlx::query_as::<_, (String, i32)>(
        r#"SELECT
g.type AS geometry_type,
g.srid AS epsg_id
FROM geometry_columns g
WHERE g.f_table_schema = $1
AND g.f_table_name = $2
AND g.f_geometry_column = 'geom'"#,
    )
    .bind(&schema)
    .bind(&table)
    .fetch_optional(&mut *conn)
    .await?
    {
        use domain::figure_layer::SupportedEpsg;
        match WkbType::from_str(&geom_type) {
            Ok(wkb_type) => {
                let valid_epsg_id = match epsg_id {
                    27700 => SupportedEpsg::BNG,
                    4326 => SupportedEpsg::WGS84,
                    _ => {
                        return Ok(ProjectLayer::Invalid(PgTableInvalidOutputDTO {
                            table,
                            schema,
                            message: format!("Layer has unsupported epsg id: {}", epsg_id),
                        }));
                    }
                };
                ProjectLayer::Valid(PgTableOutputDTO {
                    table,
                    schema,
                    geometry_type: (&wkb_type).into(),
                    wkb_type,
                    epsg_id: valid_epsg_id,
                })
            }
            Err(e) => ProjectLayer::Invalid(PgTableInvalidOutputDTO {
                table,
                schema,
                message: format!("layer has invalid geometry type: {}", e),
            }),
        }
    } else {
        ProjectLayer::Invalid(PgTableInvalidOutputDTO {
            table,
            schema,
            message: "Layer not found in database".into(),
        })
    };

    Ok(ds)
}

pub async fn select_all_layers_for_figure(
    conn: &mut PgConnection,
    figure_id: FigureId,
) -> Result<Vec<FigureLayerOutputDTO>, RepositoryError> {
    let mut layers = Vec::new();
    let res: Vec<FigureLayerSelection> = sqlx::query_as(
        r#"
SELECT fl.id,
       style_id,
       figure_id,
       fl.added,
       fl.added_by as user_id,
       u.first_name as added_by_first_name,
       u.last_name as added_by_last_name,
       fl.properties,
       fl.project_layer_source,
       turbine_layout_id,
       site_boundary_id,
       l.name as layout_name,
       b.name as site_boundary_name,
       fl.name,
       ls.styleqml::text
        FROM app.figure_layers fl
        JOIN app.users u ON u.id = fl.added_by
        LEFT JOIN public.layer_styles ls ON fl.style_id = ls.id
        LEFT JOIN app.project_features b ON b.id = fl.site_boundary_id
        LEFT JOIN app.turbine_layouts l ON l.id = fl.turbine_layout_id
        WHERE figure_id = $1
        ORDER BY layer_order"#,
    )
    .bind(figure_id.0)
    .fetch_all(&mut *conn)
    .await?;
    for row in res {
        let ds = if let Some(id) = row.site_boundary_id {
            FigureLayerDatasourceOutput::SiteBoundary(SiteAssetDatasourceOutputDTO {
                id: SiteAssetId::BoundryId(id),
                name: row.site_boundary_name,
            })
        } else if let Some(id) = row.turbine_layout_id {
            FigureLayerDatasourceOutput::TurbineLayout(SiteAssetDatasourceOutputDTO {
                id: SiteAssetId::TurbineLayout(id),
                name: row.layout_name,
            })
        } else if let Some(sqlx::types::Json(pg_table)) = row.project_layer_source {
            FigureLayerDatasourceOutput::ProjectLayer(
                select_project_layer_datasource(pg_table, &mut *conn).await?,
            )
        } else {
            todo!()
        };

        let bounding_box = bounding_box(&ds, &mut *conn).await?;

        let layer = FigureLayerOutputDTO {
            id: row.id,
            style_id: row.style_id,
            figure_id: row.figure_id,
            name: row.name,
            properties: row.properties.0,
            source: ds,
            bounding_box,
            user_id: row.user_id,
            added_by_first_name: row.added_by_first_name,
            added_by_last_name: row.added_by_last_name,
            added: row.added,
            styleqml: row.styleqml,
        };
        layers.push(layer);
    }
    Ok(layers)
}

const EXTENT_QUERY: &str = "SELECT ST_XMin(extent) as min_x,
        ST_YMin(extent) as min_y,
        ST_XMax(extent) as max_x,
        ST_YMax(extent) as max_y,
        srid
   FROM cte";

const BOUNDARY_CTE: &str = "WITH cte AS (
         SELECT
      ST_Extent(geom) as extent,
      (SELECT ST_SRID(geom) FROM app.project_features WHERE id = $1) as srid
  FROM app.project_features
                WHERE id = $1)";

const LAYOUT_CTE: &str = "WITH cte AS (SELECT ST_Extent(geom) as extent, (SELECT ST_SRID(geom) FROM app.turbines WHERE layout_id = $1 LIMIT 1) as srid FROM app.turbines WHERE layout_id = $1)";

async fn bounding_box(
    datasource: &FigureLayerDatasourceOutput,
    conn: &mut PgConnection,
) -> Result<Option<BoundingBox>, RepositoryError> {
    let query_sql;

    let query = match datasource {
        FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Valid(PgTableOutputDTO {
            table,
            schema,
            ..
        })) => {
            query_sql = format!(
                r#"WITH cte AS (
      SELECT ST_Extent(geom) as extent,
             Find_SRID('{0}', '{1}', 'geom') as srid
        FROM "{0}"."{1}") {2}"#,
                schema, table, EXTENT_QUERY,
            );
            Some(sqlx::query(&query_sql))
        }
        FigureLayerDatasourceOutput::SiteBoundary(ds) => {
            let id_val = match ds.id {
                SiteAssetId::BoundryId(f) => f.0,
                _ => unreachable!(),
            };
            query_sql = format!("{} {}", BOUNDARY_CTE, EXTENT_QUERY);
            Some(sqlx::query(&query_sql).bind(id_val))
        }
        FigureLayerDatasourceOutput::TurbineLayout(ds) => {
            let id_val = match ds.id {
                SiteAssetId::TurbineLayout(l) => l.0,
                _ => unreachable!(),
            };
            query_sql = format!("{} {}", LAYOUT_CTE, EXTENT_QUERY);
            Some(sqlx::query(&query_sql).bind(id_val))
        }
        _ => None,
    };
    if let Some(query) = query {
        let res = query.fetch_one(conn).await?;
        Ok(BoundingBox::from_row(&res).ok())
    } else {
        Ok(None)
    }
}
