use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Serialize, Deserialize)]
pub enum Status {
    Active,
    Archived,
    Deleted,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[sqlx(type_name = "geometry_type", rename_all = "UPPERCASE")]
pub enum GeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
}
