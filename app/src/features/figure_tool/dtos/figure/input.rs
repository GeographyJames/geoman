use domain::{ProjectId, UserId, enums::Status};

use crate::{
    config::QgisFigureConfig,
    features::{data_providers::DataProviderLayerId, figure_tool::dtos::FigureLayerInputDTO},
};

use super::FigureProperties;

#[derive(Debug)]
pub struct FigureInputDTO {
    pub project_id: ProjectId,
    pub properties: FigureProperties,
    pub main_map_base_map_id: Option<DataProviderLayerId>,
    pub overvier_map_base_map_id: Option<DataProviderLayerId>,
    pub qgis_project_uuid: uuid::Uuid,
    pub user_id: UserId,
    pub status: Status,
    pub page_width_mm: u32,
    pub page_height_mm: u32,
    pub scale: u32,
    pub legend_width_mm: u32,
    pub margin_mm: u32,
    pub srid: u16,
    pub layers: Vec<FigureLayerInputDTO>,
    pub figure_config: Option<QgisFigureConfig>,
}
