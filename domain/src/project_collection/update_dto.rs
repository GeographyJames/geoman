use crate::ProjectCollectionId;

pub struct CollectionUpdateDto {
    pub id: ProjectCollectionId,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
}
