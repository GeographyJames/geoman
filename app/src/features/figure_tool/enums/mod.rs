use std::str::FromStr;

use domain::{FeatureId, LayoutId};
use serde::{Deserialize, Deserializer, Serialize};
use strum::EnumString;

use super::dtos::{PgTableInputDTO, PgTableInvalidOutputDTO, PgTableOutputDTO};

#[derive(Serialize, Debug, Default, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "lowercase")]
pub enum CopyrightText {
    #[default]
    Default,
    Custom,
    None,
}

impl<'de> Deserialize<'de> for CopyrightText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        CopyrightText::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(EnumString, Serialize, Clone, sqlx::Type, Debug, PartialEq)]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "figure_status", rename_all = "lowercase")]
pub enum FigureStatus {
    ACTIVE,
    DELETED,
    ARCHIVED,
}

impl<'de> Deserialize<'de> for FigureStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum FigureLayerDatasourceInput {
    PgTable(PgTableInputDTO),
    SiteBoundary(FeatureId),
    TurbineLayout(LayoutId),
}

impl FigureLayerDatasourceInput {
    pub fn site_boundary_id(&self) -> Option<FeatureId> {
        match self {
            FigureLayerDatasourceInput::SiteBoundary(id) => Some(*id),
            _ => None,
        }
    }
    pub fn turbine_layout_id(&self) -> Option<LayoutId> {
        match self {
            FigureLayerDatasourceInput::TurbineLayout(id) => Some(*id),
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
pub enum FigureLayerDatasourceOutput {
    ProjectLayer(ProjectLayer),
    SiteBoundary(SiteBoundaryDatasourceOutputDTO),
    TurbineLayout(TurbineLayoutDatasourceOutputDTO),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteBoundaryDatasourceOutputDTO {
    pub id: FeatureId,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurbineLayoutDatasourceOutputDTO {
    pub id: LayoutId,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectLayer {
    Valid(PgTableOutputDTO),
    Invalid(PgTableInvalidOutputDTO),
}
