use qgis::{
    layer::{Geometry, WkbType},
    srs::SupportedEpsg,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct PgTableOutputDTO {
    pub table: String,
    pub schema: String,
    pub wkb_type: WkbType,
    pub geometry_type: Geometry,
    pub epsg_id: SupportedEpsg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgTableInvalidOutputDTO {
    pub table: String,
    pub schema: String,
    pub message: String,
}
