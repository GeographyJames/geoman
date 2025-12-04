use domain::{
    enums::Visibility,
    project::{ProjectInputDto, ProjectNameInputDTO, ProjectSlug},
};
use serde::{Deserialize, Serialize};

use crate::handlers::{ApiError, ProjectValidationError};

#[derive(Serialize, Deserialize)]
pub struct ProjectReqPayload {
    pub name: String,
    pub visibility: Visibility,
    pub country_code: String,
    pub crs_srid: Option<i32>,
}

impl ProjectReqPayload {
    pub fn try_into_dto(self) -> Result<ProjectInputDto, ApiError> {
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
            country_code: isocountry::CountryCode::for_alpha2(&country_code)
                .map_err(ProjectValidationError::InvalidCountryCode)?,
            crs_srid,
        })
    }
}
