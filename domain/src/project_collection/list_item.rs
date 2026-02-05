use crate::{ProjectCollectionId, enums::GeometryType};
use serde::Serialize;

#[derive(Serialize)]
pub struct CollectionListItem {
    pub id: ProjectCollectionId,
    pub title: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
}
