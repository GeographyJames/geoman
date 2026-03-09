use crate::{
    app::configuration::QgisFigureConfig,
    domain::{
        dtos::{FigureLayerInputDTO, FigureProperties, Id, UserId},
        enums::FigureStatus,
    },
};

#[derive(Debug)]
pub struct FigureInputDTO {
    pub project_id: Id,
    pub properties: FigureProperties,
    pub main_map_base_map_id: Option<Id>,
    pub overvier_map_base_map_id: Option<Id>,
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
