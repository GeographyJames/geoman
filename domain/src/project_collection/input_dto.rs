use crate::{ProjectId, enums::GeometryType, name::NameInputDTO};

pub struct ProjectCollectionInputDto {
    pub title: NameInputDTO,
    pub slug: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
    pub project_id: Option<ProjectId>,
}
