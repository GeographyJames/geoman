use crate::{enums::GeometryType, name::NameInputDTO};

pub struct ProjectCollectionInputDto {
    pub title: NameInputDTO,
    pub slug: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
}
