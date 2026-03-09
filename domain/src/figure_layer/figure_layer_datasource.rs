use serde::{Deserialize, Serialize};

use crate::{
    FeatureId, LayoutId,
    pg_table::{PgTableInputDTO, PgTableInvalidOutputDTO, PgTableOutputDTO},
};

#[derive(Debug, Clone, Serialize)]
pub enum FigureLayerDatasourceInput {
    PgTable(PgTableInputDTO),
    SiteBoundary(FeatureId),
    TurbineLayout(LayoutId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FigureLayerDatasourceOutput {
    // ProjectLayer(ProjectLayer),
    SiteBoundary(SiteAssetDatasourceOutputDTO),
    TurbineLayout(SiteAssetDatasourceOutputDTO),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteAssetDatasourceOutputDTO {
    pub id: SiteAssetId,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SiteAssetId {
    BoundryId(FeatureId),
    TurbineLayout(LayoutId),
}
impl FigureLayerDatasourceInput {
    pub fn site_boundary_id(&self) -> Option<FeatureId> {
        match self {
            FigureLayerDatasourceInput::SiteBoundary(id) => Some(id.to_owned()),
            _ => None,
        }
    }
    pub fn turbine_layout_id(&self) -> Option<LayoutId> {
        match self {
            FigureLayerDatasourceInput::TurbineLayout(id) => Some(id.to_owned()),
            _ => None,
        }
    }
    pub fn project_layer_src(&self) -> Option<PgTableInputDTO> {
        match self {
            FigureLayerDatasourceInput::PgTable(src) => Some(src.to_owned()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectLayer {
    Valid(PgTableOutputDTO),
    Invalid(PgTableInvalidOutputDTO),
}
