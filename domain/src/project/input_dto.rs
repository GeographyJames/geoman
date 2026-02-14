use crate::{
    enums::Visibility,
    project::{ProjectNameInputDTO, ProjectSlugInputDto},
};

#[derive(Debug)]
pub struct ProjectInputDto {
    pub name: ProjectNameInputDTO,
    pub visibility: Visibility,
    pub crs_srid: Option<i32>,
    pub slug: ProjectSlugInputDto,
}
