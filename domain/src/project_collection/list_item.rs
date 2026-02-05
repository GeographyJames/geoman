use crate::{ProjectCollectionId, enums::GeometryType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CollectionListItem {
    pub id: ProjectCollectionId,
    pub title: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
    pub feature_count: i64,
}
