use anyhow::Context;
use chrono::Utc;
use gdal::spatial_ref::{CoordTransform, SpatialRef};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection, PgPool};

use crate::{
    domain::{
        dtos::{
            BaseMapOutputDTO, BoundingBox, FigureLayerOutputDTO, FigureOutputDTO, FigureProperties,
            Id, OVERVIEW_MAP_SCALE, Point, UserId,
        },
        enums::FigureStatus,
    },
    postgres::figure_layer::select_all_layers_for_figure,
    qgis::{Extent, layout::components::SizeInteger},
    repo::{RepositoryError, Select, SelectAllForProject},
};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
struct FigureSelection {
    id: Id,
    project_id: Id,
    project_name: String,
    main_map_base_map_id: Option<Id>,
    overview_map_base_map_id: Option<Id>,
    qgis_project_uuid: uuid::Uuid,
    added_by: UserId,
    added_by_first_name: String,
    added_by_last_name: String,
    last_updated_by: UserId,
    last_updated_by_first_name: String,
    last_updated_by_last_name: String,
    status: FigureStatus,
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

impl<'a> SelectAllForProject<&'a PgPool, Id> for FigureOutputDTO {
    async fn select_all_for_project(
        executor: &'a PgPool,
        project_id: &Id,
    ) -> Result<Vec<Self>, crate::repo::RepositoryError> {
        let mut figures = Vec::new();
        let mut tx = executor.begin().await?;
        let res: Vec<FigureSelection> = sqlx::query_as(&format!(
            "{} WHERE f.project_id = $1 AND f.status != 'deleted'",
            BASE_QUERY
        ))
        .bind(project_id.as_ref())
        .fetch_all(&mut *tx)
        .await?;
        for row in res {
            figures.push(FigureOutputDTO::from_figure_selection(row, &mut tx).await?)
        }
        Ok(figures)
    }
}

impl Select<&mut PgConnection, Id> for FigureOutputDTO {
    async fn select(
        conn: &mut PgConnection,
        id: &Id,
    ) -> Result<Self, crate::repo::RepositoryError> {
        let res: FigureSelection = sqlx::query_as(&format!("{} WHERE f.id = $1", BASE_QUERY))
            .bind(id.as_ref())
            .fetch_one(&mut *conn)
            .await?;
        let figure = FigureOutputDTO::from_figure_selection(res, &mut *conn).await?;

        Ok(figure)
    }
}

impl FigureOutputDTO {
    async fn from_figure_selection(
        row: FigureSelection,
        tx: &mut PgConnection,
    ) -> Result<Self, crate::repo::RepositoryError> {
        let layers = Some(select_all_layers_for_figure(&mut *tx, &row.id).await?);
        let target_layer_bounds = if let Some(ref l) = layers {
            target_layers_bounds(l)
                .context("failed to calculate target layer bounds")
                .map_err(|e| RepositoryError::UnexpectedError(e.into()))?
        } else {
            None
        };
        let target_coord = row
            .target_coords(&target_layer_bounds)
            .context("failed to get target coords")
            .map_err(|e| RepositoryError::UnexpectedError(e.into()))?;
        let map_extent = row.map_extent(target_coord);
        let overview_map_extent = row.overview_map_extent(target_coord);
        let FigureSelection {
            id,
            project_id,
            project_name,
            qgis_project_uuid,
            added_by,
            added_by_first_name,
            added_by_last_name,
            last_updated_by,
            last_updated_by_first_name,
            last_updated_by_last_name,
            status,
            added,
            last_updated,
            page_width_mm,
            page_height_mm,
            margin_mm,
            legend_width_mm,
            scale,
            srid,
            properties,
            main_map_base_map_id,
            overview_map_base_map_id,
        } = row;
        let main_map_base_map = if let Some(id) = main_map_base_map_id {
            Some(BaseMapOutputDTO::select(&mut *tx, &id).await?)
        } else {
            None
        };
        let overview_map_base_map = if let Some(id) = overview_map_base_map_id {
            Some(BaseMapOutputDTO::select(&mut *tx, &id).await?)
        } else {
            None
        };
        let target_layer_bounds_27700 = if let Some(b) = target_layer_bounds {
            if b.srid != 27700 {
                Some(
                    b.transform(27700)
                        .map_err(|e| RepositoryError::UnexpectedError(e.into()))?,
                )
            } else {
                Some(b)
            }
        } else {
            None
        };

        Ok(Self {
            id,
            main_map_base_map_id,
            overview_map_base_map_id,
            project_id,
            project_name,
            qgis_project_uuid,
            added_by,
            added_by_first_name,
            added_by_last_name,
            last_updated_by,
            last_updated_by_first_name,
            last_updated_by_last_name,
            status,
            added,
            last_updated,
            page_width_mm,
            page_height_mm,
            margin_mm,
            legend_width_mm,
            scale,
            srid,
            properties: properties.0,
            layers,
            main_map_base_map,
            overview_map_base_map,
            map_extent,
            target_coord,
            overview_map_extent,
            target_layer_extent: target_layer_bounds_27700.map(|b| Extent {
                xmin: b.min_x,
                ymin: b.min_y,
                xmax: b.max_x,
                ymax: b.max_y,
            }),
        })
    }
}

pub fn target_layers_bounds(
    layers: &[FigureLayerOutputDTO],
) -> Result<Option<BoundingBox>, anyhow::Error> {
    let mut target_box = None;

    let mut target_layers = layers
        .iter()
        .filter(|l| l.properties.include_as_target)
        .filter_map(|l| l.bounding_box.map(|bbox| (bbox, l.name.clone())))
        .collect::<Vec<_>>()
        .into_iter();

    if let Some((bbox, _layer_name)) = target_layers.next() {
        let mut target = bbox;
        for (bbox, _) in target_layers {
            target = target.combine(&bbox)?;
        }
        target_box = Some(target)
    }

    Ok(target_box)
}

impl FigureSelection {
    pub fn target_coords(
        &self,
        target_layer_bounds: &Option<BoundingBox>,
    ) -> Result<Point, anyhow::Error> {
        if let (Some(x), Some(y)) = (
            self.properties.target_x_coordinate,
            self.properties.target_y_coordinate,
        ) {
            return Ok(Point {
                x,
                y,
                srid: self.srid as u32,
            });
        }
        let point = if let Some(target_layer_bounds) = target_layer_bounds {
            let transformed = target_layer_bounds
                .transform(self.srid as u32)
                .context("failed to convert target layer bounds CRS to figure CRS")?;
            transformed.centre()
        } else {
            let mut gdal_point = gdal::vector::Geometry::from_wkt("POINT (324636 673221)")
                .context("failed to initialise default target point")?;
            if self.srid != 27700 {
                let targe_crs = SpatialRef::from_epsg(self.srid as u32)
                    .context("failed to initialise target CRS")?;
                let mut source_crs =
                    SpatialRef::from_epsg(27700).context("failed to initialise source crs")?;
                source_crs.set_axis_mapping_strategy(
                    gdal::spatial_ref::AxisMappingStrategy::TraditionalGisOrder,
                );
                let trans = CoordTransform::new(&source_crs, &targe_crs)
                    .context("failed to initialise coordinate transform")?;
                gdal_point
                    .transform_inplace(&trans)
                    .context("failed to transform default target point")?;
            }
            let (x, y, _) = gdal_point.get_point(0);
            Point {
                x,
                y,
                srid: self.srid as u32,
            }
        };

        Ok(Point {
            x: point.x.round(),
            y: point.y.round(),
            srid: self.srid as u32,
        })
    }

    pub fn map_right(&self) -> u32 {
        if self.properties.0.legend_height_percent.unwrap_or(100) < 100 {
            (self.page_width_mm - self.margin_mm) as u32
        } else {
            (self.page_width_mm - self.margin_mm - self.legend_width_mm) as u32
        }
    }
    pub fn map_left(&self) -> u32 {
        self.margin_mm as u32
    }
    pub fn map_top(&self) -> u32 {
        self.margin_mm as u32
    }
    pub fn map_bottom(&self) -> u32 {
        (self.page_height_mm - self.margin_mm) as u32
    }
    pub fn map_width(&self) -> u32 {
        self.map_right() - self.map_left()
    }
    pub fn map_height(&self) -> u32 {
        self.map_bottom() - self.map_top()
    }
    pub fn map_extent(&self, target_coordinate: Point) -> Extent {
        extent_calculator(
            self.scale as u32,
            SizeInteger {
                width_mm: self.map_width(),
                height_mm: self.map_height(),
            },
            target_coordinate,
        )
    }

    pub fn overview_map_extent(&self, target_coordinate: Point) -> Extent {
        extent_calculator(
            self.properties
                .0
                .overview_map_scale
                .unwrap_or(OVERVIEW_MAP_SCALE),
            SizeInteger {
                width_mm: self.legend_width_mm as u32,
                height_mm: self.legend_width_mm as u32,
            },
            target_coordinate,
        )
    }
}

fn extent_calculator(scale: u32, size: SizeInteger, target_coordinate: Point) -> Extent {
    let width_at_scale_m = (size.width_mm as f64 / 1000.) * scale as f64;
    let height_at_scale_m = (size.height_mm as f64 / 1000.) * scale as f64;
    let half_map_height_at_scale_m = height_at_scale_m / 2.;
    let half_map_width_at_scale_m = width_at_scale_m / 2.;

    let Point { x, y, srid: _ } = target_coordinate;

    let xmin = x - half_map_width_at_scale_m;
    let xmax = x + half_map_width_at_scale_m;
    let ymin = y - half_map_height_at_scale_m;
    let ymax = y + half_map_height_at_scale_m;

    Extent {
        xmax,
        xmin,
        ymax,
        ymin,
    }
}
