use domain::{
    enums::Visibility,
    project::{ProjectInputDto, ProjectNameInputDTO, ProjectSlug},
};
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

use crate::handlers::ProjectValidationError;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectReqPayload {
    pub name: String,
    pub visibility: Visibility,
    pub country_code: String,
    pub crs_srid: Option<i32>,
}

impl Default for ProjectReqPayload {
    fn default() -> Self {
        Self {
            name: uuid::Uuid::new_v4().to_string(),
            visibility: Default::default(),
            country_code: CountryCode::GBR.alpha2().to_string(),
            crs_srid: Default::default(),
        }
    }
}

impl TryInto<ProjectInputDto> for ProjectReqPayload {
    fn try_into(self) -> Result<ProjectInputDto, ProjectValidationError> {
        let ProjectReqPayload {
            name,
            visibility,
            country_code,
            crs_srid,
        } = self;
        let slug = ProjectSlug::parse(&name);
        let name =
            ProjectNameInputDTO::parse(name).map_err(ProjectValidationError::InvalidProjectName)?;
        Ok(ProjectInputDto {
            slug,
            name,
            visibility,
            country_code: isocountry::CountryCode::for_alpha2(&country_code)?,
            crs_srid,
        })
    }

    type Error = ProjectValidationError;
}
