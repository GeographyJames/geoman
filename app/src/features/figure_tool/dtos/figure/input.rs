use crate::{
    app::{
        configuration::QgisFigureConfig,
        features::figure_tool::{
            dtos::figure_layer::FigureLayerInputDTO,
            enums::FigureStatus,
            ids::{BaseMapId, ProjectId},
        },
    },
    domain::dtos::UserId,
};

use super::FigureProperties;

#[derive(Debug)]
pub struct FigureInputDTO {
    pub project_id: ProjectId,
    pub properties: FigureProperties,
    pub main_map_base_map_id: Option<BaseMapId>,
    pub overvier_map_base_map_id: Option<BaseMapId>,
    pub qgis_project_uuid: uuid::Uuid,
    pub user_id: UserId,
    pub status: FigureStatus,
    pub page_width_mm: u32,
    pub page_height_mm: u32,
    pub scale: u32,
    pub legend_width_mm: u32,
    pub margin_mm: u32,
    pub srid: u16,
    pub layers: Vec<FigureLayerInputDTO>,
    pub figure_config: Option<QgisFigureConfig>,
}
