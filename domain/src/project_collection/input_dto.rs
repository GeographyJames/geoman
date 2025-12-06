use crate::enums::GeometryType;

pub struct ProjectCollectionInputDto {
    pub title: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
}
