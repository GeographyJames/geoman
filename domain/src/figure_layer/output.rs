use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::domain::{
    dtos::{BoundingBox, Id, LayerProperties, UserId},
    enums::FigureLayerDatasourceOutput,
};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FigureLayerOutputDTO {
    pub id: Id,
    pub style_id: Option<Id>,
    pub figure_id: Id,
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
