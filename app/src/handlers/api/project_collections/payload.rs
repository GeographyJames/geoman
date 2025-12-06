use domain::{ProjectCollectionInputDto, enums::GeometryType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CollectionReqPayload {
    pub title: String,
    pub geometry_type: GeometryType,
    pub description: Option<String>,
}

impl Default for CollectionReqPayload {
    fn default() -> Self {
        Self {
            title: uuid::Uuid::new_v4().to_string(),
            geometry_type: GeometryType::Point,
            description: Default::default(),
        }
    }
}

impl From<CollectionReqPayload> for ProjectCollectionInputDto {
    fn from(value: CollectionReqPayload) -> ProjectCollectionInputDto {
        let CollectionReqPayload {
            title,
            geometry_type,
            description,
        } = value;
        ProjectCollectionInputDto {
            title,
            description,
            geometry_type,
        }
    }
}
