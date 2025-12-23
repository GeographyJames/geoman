use crate::{
    TechnologyId,
    enums::Visibility,
    project::{ProjectNameInputDTO, ProjectSlugInputDto},
};

#[derive(Debug)]
pub struct ProjectInputDto {
    pub name: ProjectNameInputDTO,
    pub visibility: Visibility,
    pub country_code: isocountry::CountryCode,
    pub crs_srid: Option<i32>,
    pub slug: ProjectSlugInputDto,
    pub technologies: Option<Vec<TechnologyId>>,
}
