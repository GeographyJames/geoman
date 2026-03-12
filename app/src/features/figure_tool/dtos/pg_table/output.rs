use serde::{Deserialize, Serialize};

use crate::{
    app::features::figure_tool::enums::SupportedEpsg,
    qgis::layer::{Geometry, WkbType},
};

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
