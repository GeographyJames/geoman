use anyhow::Context;
use gdal::{
    spatial_ref::{CoordTransform, SpatialRef},
    vector::{Envelope, Geometry},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub srid: u32,
}

#[derive(Serialize, Debug, Deserialize, Clone, Copy, sqlx::Type, FromRow)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub srid: i32,
}

impl BoundingBox {
    pub fn transform(self, target_srid: u32) -> Result<BoundingBox, anyhow::Error> {
        if self.srid as u32 == target_srid {
            return Ok(self);
        }
        let source_srid = self.srid;
        let mut gdal_bbox: Geometry = self.try_into()?;

        let mut source_srs =
            SpatialRef::from_epsg(source_srid as u32).context("failed to initialise source srs")?;
        source_srs
            .set_axis_mapping_strategy(gdal::spatial_ref::AxisMappingStrategy::TraditionalGisOrder);
        gdal_bbox.set_spatial_ref(source_srs.clone());

        let mut target_srs =
            SpatialRef::from_epsg(target_srid).context("failed to initialise target srs")?;
        target_srs
            .set_axis_mapping_strategy(gdal::spatial_ref::AxisMappingStrategy::TraditionalGisOrder);
        gdal_bbox.set_spatial_ref(source_srs.clone());
        let trans = CoordTransform::new(&source_srs, &target_srs)
            .context("failed to initialise coordinate transform")?;
        gdal_bbox
            .transform_inplace(&trans)
            .context("failed to transform geometry")?;
        let Envelope {
            MinX,
            MaxX,
            MinY,
            MaxY,
        } = gdal_bbox.envelope();
        Ok(BoundingBox {
            max_x: MaxX,
            min_x: MinX,
            max_y: MaxY,
            min_y: MinY,
            srid: target_srid as i32,
        })
    }

    pub fn combine(&self, other: &BoundingBox) -> Result<BoundingBox, anyhow::Error> {
        let transformed;
        let input = if self.srid != other.srid {
            transformed = other
                .transform(self.srid as u32)
                .context("failed to transform bounding box")?;
            &transformed
        } else {
            other
        };

        Ok(BoundingBox {
            min_x: self.min_x.min(input.min_x),
            min_y: self.min_y.min(input.min_y),
            max_x: self.max_x.max(input.max_x),
            max_y: self.max_y.max(input.max_y),
            srid: self.srid,
        })
    }

    pub fn centre(&self) -> Point {
        Point {
            x: self.min_x + (self.max_x - self.min_x) / 2.,
            y: self.min_y + (self.max_y - self.min_y) / 2.,
            srid: self.srid as u32,
        }
    }
}

impl TryInto<Geometry> for BoundingBox {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Geometry, anyhow::Error> {
        gdal::vector::Geometry::bbox(self.min_x, self.min_y, self.max_x, self.max_y)
            .context("failed to create geometry from Bounding Box")
    }
}
