use domain::{
    ProjectId, TechnologyId,
    enums::{Status, Visibility},
    project::{ProjectInputDto, ProjectNameInputDTO, ProjectSlugInputDto, ProjectUpdateDto},
};
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

use crate::{errors::ApiError, handlers::ProjectValidationError};

#[derive(Serialize, Deserialize, Clone)]
pub struct PostProjectPayload {
    pub name: String,
    pub visibility: Option<Visibility>,
    pub country_code: String,
    pub crs_srid: Option<i32>,
    pub slug: String,
    pub technologies: Option<Vec<TechnologyId>>,
}

impl Default for PostProjectPayload {
    fn default() -> Self {
        let name = uuid::Uuid::new_v4().to_string();
        Self {
            slug: name.clone(),
            name,
            visibility: Default::default(),
            country_code: CountryCode::GBR.alpha2().to_string(),
            crs_srid: Default::default(),
            technologies: Default::default(),
        }
    }
}

impl TryInto<ProjectInputDto> for PostProjectPayload {
    fn try_into(self) -> Result<ProjectInputDto, ProjectValidationError> {
        let PostProjectPayload {
            name,
            visibility,
            country_code,
            crs_srid,
            slug,
            technologies,
        } = self;
        let slug = ProjectSlugInputDto::try_from(slug)
            .map_err(ProjectValidationError::InvalidProjectSlug)?;
        let name =
            ProjectNameInputDTO::parse(name).map_err(ProjectValidationError::InvalidProjectName)?;
        Ok(ProjectInputDto {
            slug,
            name,
            visibility: visibility.unwrap_or(Visibility::Private),
            country_code: isocountry::CountryCode::for_alpha2(&country_code)?,
            crs_srid,
            technologies,
        })
    }

    type Error = ProjectValidationError;
}

#[derive(Default, Serialize, Deserialize)]
pub struct PatchProjectPayload {
    pub status: Option<Status>,
}

impl PatchProjectPayload {
    pub fn try_into_dto(self, project_id: ProjectId) -> Result<ProjectUpdateDto, ApiError> {
        let PatchProjectPayload { status } = self;
        Ok(ProjectUpdateDto {
            status,
            id: project_id,
        })
    }
}
