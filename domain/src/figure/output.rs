use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    DataProviderLayerId, FigureId, ProjectId, UserId,
    enums::Status,
    figure::FigureProperties,
    figure_layer::FigureLayerOutputDTO,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FigureOutputDTO {
    pub id: FigureId,
    pub project_id: ProjectId,
    pub project_name: String,
    pub main_map_base_map_id: Option<DataProviderLayerId>,
    pub overview_map_base_map_id: Option<DataProviderLayerId>,
    pub qgis_project_uuid: uuid::Uuid,
    pub added_by: UserId,
    pub added_by_first_name: String,
    pub added_by_last_name: String,
    pub last_updated_by: UserId,
    pub last_updated_by_first_name: String,
    pub last_updated_by_last_name: String,
    pub status: Status,
    pub added: chrono::DateTime<Utc>,
    pub last_updated: chrono::DateTime<Utc>,
    pub page_width_mm: i32,
    pub page_height_mm: i32,
    pub margin_mm: i32,
    pub legend_width_mm: i32,
    pub scale: i32,
    pub srid: i32,
    pub properties: FigureProperties,
    pub layers: Option<Vec<FigureLayerOutputDTO>>,
}
