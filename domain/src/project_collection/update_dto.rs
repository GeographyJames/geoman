use crate::{ProjectCollectionId, enums::Status};

pub struct CollectionUpdateDto {
    pub id: ProjectCollectionId,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<Status>,
}
