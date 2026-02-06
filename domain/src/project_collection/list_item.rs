use chrono::{DateTime, Utc};

use crate::{AddedBy, ProjectCollectionId, enums::GeometryType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CollectionListItem {
    pub id: ProjectCollectionId,
    pub title: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
    pub active_feature_count: i64,
    pub archived_feature_count: i64,
    pub added: DateTime<Utc>,
    #[serde(flatten)]
    pub added_by: AddedBy,
}
