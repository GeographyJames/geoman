use std::str;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

type Bbox2D = [f64; 4];
type Bbox3D = [f64; 6];

/// Each bounding box is provided as four or six numbers, depending on
/// whether the coordinate reference system includes a vertical axis
/// (height or depth):
///
/// * Lower left corner, coordinate axis 1
///
/// * Lower left corner, coordinate axis 2
///
/// * Minimum value, coordinate axis 3 (optional)
///
/// * Upper right corner, coordinate axis 1
///
/// * Upper right corner, coordinate axis 2
///
/// * Maximum value, coordinate axis 3 (optional)
///
/// The coordinate reference system of the values is WGS 84 longitude/latitude
/// (http://www.opengis.net/def/crs/OGC/1.3/CRS84) unless a different coordinate
/// reference system is specified in `crs`.
///
/// For WGS 84 longitude/latitude the values are in most cases the sequence of
/// minimum longitude, minimum latitude, maximum longitude and maximum latitude.
/// However, in cases where the box spans the antimeridian the first value
/// (west-most box edge) is larger than the third value (east-most box edge).
///
/// If the vertical axis is included, the third and the sixth number are
/// the bottom and the top of the 3-dimensional bounding box.
///
/// If a feature has multiple spatial geometry properties, it is the decision of the
/// server whether only a single spatial geometry property is used to determine
/// the extent or all relevant geometries.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Bbox {
    #[schema(value_type = Vec<f64>)]
    Bbox2D(Bbox2D),
    #[schema(value_type = Vec<f64>)]
    Bbox3D(Bbox3D),
}
