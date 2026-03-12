use chrono::Utc;
use domain::UserId;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::features::figure_tool::{
    FigureLayerDatasourceOutput,
    dtos::BoundingBox,
    ids::{FigureId, FigureLayerId, LayerStyleId},
};

use super::LayerProperties;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FigureLayerOutputDTO {
    pub id: FigureLayerId,
    pub style_id: Option<LayerStyleId>,
    pub figure_id: FigureId,
    pub name: String,

    pub source: FigureLayerDatasourceOutput,

    pub bounding_box: Option<BoundingBox>,
    pub user_id: UserId,
    pub added_by_first_name: String,
    pub added_by_last_name: String,
    pub added: chrono::DateTime<Utc>,
    pub properties: LayerProperties,
    #[serde(skip_serializing)]
    pub styleqml: Option<String>,
}
