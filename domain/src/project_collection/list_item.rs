use crate::{ProjectCollectionId, enums::GeometryType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CollectionListItem {
    pub id: ProjectCollectionId,
    pub title: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
    pub active_feature_count: i64,
    pub archived_feature_count: i64,
}
