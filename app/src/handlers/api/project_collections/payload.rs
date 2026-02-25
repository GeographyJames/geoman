use domain::{ProjectCollectionInputDto, ProjectId, enums::GeometryType, name::NameInputDTO};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CollectionReqPayload {
    pub title: String,
    pub project_id: Option<ProjectId>,
    pub geometry_type: GeometryType,
    pub description: Option<String>,
}

impl Default for CollectionReqPayload {
    fn default() -> Self {
        Self {
            title: uuid::Uuid::new_v4().to_string(),
            geometry_type: GeometryType::Point,
            description: Default::default(),
            project_id: None,
        }
    }
}

impl TryFrom<CollectionReqPayload> for ProjectCollectionInputDto {
    fn try_from(value: CollectionReqPayload) -> Result<ProjectCollectionInputDto, String> {
        let CollectionReqPayload {
            title,
            geometry_type,
            description,
            project_id,
        } = value;
        let slug = slug::slugify(&title);
        Ok(ProjectCollectionInputDto {
            title: NameInputDTO::parse(title)?,
            slug,
            description,
            geometry_type,
            project_id,
        })
    }

    type Error = String;
}
