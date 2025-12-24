use crate::{ProjectId, enums::Status};

pub struct ProjectUpdateDto {
    pub id: ProjectId,
    pub status: Option<Status>,
}
