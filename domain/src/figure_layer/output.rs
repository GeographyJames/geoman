use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    FigureId, FigureLayerId, StyleId, UserId,
    bounding_box::BoundingBox,
    figure_layer::{LayerProperties, figure_layer_datasource::FigureLayerDatasourceOutput},
};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FigureLayerOutputDTO {
    pub id: FigureLayerId,
    pub style_id: Option<StyleId>,
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
