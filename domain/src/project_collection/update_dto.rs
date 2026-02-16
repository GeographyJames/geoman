use crate::{ProjectCollectionId, enums::Status, name::NameInputDTO};

pub struct CollectionUpdateDto {
    pub id: ProjectCollectionId,
    pub title: Option<NameInputDTO>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<Status>,
}
