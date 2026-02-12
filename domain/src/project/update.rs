use crate::{
    ProjectId,
    enums::{Status, Visibility},
};

use super::{ProjectNameInputDTO, ProjectSlugInputDto};

pub struct ProjectUpdateDto {
    pub id: ProjectId,
    pub status: Option<Status>,
    pub name: Option<ProjectNameInputDTO>,
    pub visibility: Option<Visibility>,
    pub country_code: Option<isocountry::CountryCode>,
    pub crs_srid: Option<Option<i32>>,
    pub slug: Option<ProjectSlugInputDto>,
}
