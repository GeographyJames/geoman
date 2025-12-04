use crate::{
    enums::Visibility,
    project::{ProjectNameInputDTO, ProjectSlug},
};

#[derive(Debug)]
pub struct ProjectInputDto {
    pub name: ProjectNameInputDTO,
    pub visibility: Visibility,
    pub country_code: isocountry::CountryCode,
    pub crs_srid: Option<i32>,
    pub slug: ProjectSlug,
}
