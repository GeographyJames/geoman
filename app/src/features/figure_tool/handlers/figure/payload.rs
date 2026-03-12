use serde::{Deserialize, Serialize};

use crate::{
    app::{
        configuration::QgisFigureConfig,
        features::figure_tool::{
            dtos::{
                figure::{FigureInputDTO, FigureProperties},
                figure_layer::{FigureLayerInputDTO, LayerNameInputDTO, LayerProperties},
                pg_table::PgTableInputDTO,
            },
            enums::{FigureLayerDatasourceInput, FigureStatus},
            ids::{BaseMapId, LayerStyleId, ProjectId, SiteBoundaryId, TurbineLayoutId},
        },
    },
    domain::dtos::UserId,
    qgis::layout::{PageOrientation, PageSize, Size},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct FigurePayload {
    pub project_id: ProjectId,
    pub properties: Option<FigureProperties>,
    pub scale: Option<u32>,
    pub legend_width_mm: Option<u32>,
    pub margin_mm: Option<u32>,
    pub page_width_mm: Option<u32>,
    pub page_height_mm: Option<u32>,
    pub status: Option<FigureStatus>,
    pub layers: Option<Vec<FigureLayerPayload>>,
    pub srid: Option<u16>,
    pub main_map_base_map_id: Option<BaseMapId>,
    pub overview_map_base_map_id: Option<BaseMapId>,
}

impl FigurePayload {
    pub fn new(project_id: ProjectId) -> Self {
        FigurePayload {
            project_id,
            properties: Default::default(),
            scale: Default::default(),
            legend_width_mm: Default::default(),
            margin_mm: Default::default(),
            page_width_mm: Default::default(),
            page_height_mm: Default::default(),
            status: Default::default(),
            layers: Default::default(),
            srid: Default::default(),
            main_map_base_map_id: Default::default(),
            overview_map_base_map_id: Default::default(),
        }
    }
}

impl FigureLayerPayload {
    pub fn new(datasource: FigureLayerDatasourcePayload) -> Self {
        FigureLayerPayload {
            style_id: None,
            properties: LayerProperties::default(),
            source: datasource,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum FigureLayerDatasourcePayload {
    PgTable(PgTablePayload),
    SiteBoundary(SiteBoundaryId),
    TurbineLayout(TurbineLayoutId),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PgTablePayload {
    pub table: String,
    pub schema: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FigureLayerPayload {
    pub style_id: Option<LayerStyleId>,
    pub source: FigureLayerDatasourcePayload,
    pub properties: LayerProperties,
}

impl FigurePayload {
    pub fn into_input_dto(
        self,
        user_id: UserId,
        figure_config: Option<QgisFigureConfig>,
    ) -> Result<FigureInputDTO, String> {
        let layers = self
            .layers
            .unwrap_or_default()
            .into_iter()
            .map(|l| l.try_into())
            .collect::<Result<Vec<FigureLayerInputDTO>, String>>()?;

        Ok(FigureInputDTO {
            project_id: self.project_id,
            qgis_project_uuid: uuid::Uuid::new_v4(),
            properties: self.properties.unwrap_or_default(),
            user_id,
            status: self.status.unwrap_or(FigureStatus::ACTIVE),
            page_height_mm: self
                .page_height_mm
                .unwrap_or(Size::from(PageSize::A3(PageOrientation::Landscape)).height_mm as u32),
            page_width_mm: self
                .page_width_mm
                .unwrap_or(Size::from(PageSize::A3(PageOrientation::Landscape)).width_mm as u32),
            scale: self.scale.unwrap_or(50000),
            legend_width_mm: self.legend_width_mm.unwrap_or(70),
            margin_mm: self.margin_mm.unwrap_or(5),
            srid: self.srid.unwrap_or(27700),
            figure_config,
            main_map_base_map_id: self.main_map_base_map_id,
            overvier_map_base_map_id: self.overview_map_base_map_id,
            layers,
        })
    }
}

impl TryFrom<FigureLayerDatasourcePayload> for FigureLayerDatasourceInput {
    type Error = String;

    fn try_from(value: FigureLayerDatasourcePayload) -> Result<Self, Self::Error> {
        match value {
            FigureLayerDatasourcePayload::PgTable(pg_table_payload) => {
                Ok(FigureLayerDatasourceInput::PgTable(PgTableInputDTO::parse(
                    pg_table_payload.table,
                    pg_table_payload.schema,
                )?))
            }
            FigureLayerDatasourcePayload::SiteBoundary(id) => {
                Ok(FigureLayerDatasourceInput::SiteBoundary(id))
            }
            FigureLayerDatasourcePayload::TurbineLayout(id) => {
                Ok(FigureLayerDatasourceInput::TurbineLayout(id))
            }
        }
    }
}

impl TryFrom<FigureLayerPayload> for FigureLayerInputDTO {
    type Error = String;

    fn try_from(value: FigureLayerPayload) -> Result<FigureLayerInputDTO, Self::Error> {
        let datasource: FigureLayerDatasourceInput = value.source.try_into()?;

        let mut name = match datasource {
            FigureLayerDatasourceInput::PgTable(ref pg_table_input_dto) => {
                pg_table_input_dto.get_table().to_string()
            }
            FigureLayerDatasourceInput::SiteBoundary(id) => {
                format!("site-boundary-{}", id,)
            }
            FigureLayerDatasourceInput::TurbineLayout(id) => {
                format!("turbine-layout-{}", id,)
            }
        };
        name.push('-');
        name.push_str(&uuid::Uuid::new_v4().to_string());

        Ok(FigureLayerInputDTO {
            style_id: value.style_id,
            name: LayerNameInputDTO::parse(slug::slugify(name))?,
            properties: value.properties,
            source: datasource,
        })
    }
}
