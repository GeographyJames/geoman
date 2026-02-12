use domain::{
    ProjectId,
    enums::{Status, Visibility},
    project::{ProjectInputDto, ProjectNameInputDTO, ProjectSlugInputDto, ProjectUpdateDto},
};
use isocountry::CountryCode;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{errors::ApiError, handlers::ProjectValidationError};

/// Deserializes a field as `Some(Some(value))` when present with a value,
/// `Some(None)` when explicitly `null`, and `None` when the field is missing.
fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostProjectPayload {
    pub name: String,
    pub visibility: Option<Visibility>,
    pub country_code: String,
    pub crs_srid: Option<i32>,
    pub slug: String,
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
        } = self;
        let slug = ProjectSlugInputDto::try_from(slug)
            .map_err(ProjectValidationError::InvalidProjectSlug)?;
        let name =
            ProjectNameInputDTO::parse(name).map_err(ProjectValidationError::InvalidProjectName)?;
        let country_code = isocountry::CountryCode::for_alpha2(&country_code)
            .map_err(ProjectValidationError::InvalidCountryCode)?;

        Ok(ProjectInputDto {
            slug,
            name,
            visibility: visibility.unwrap_or(Visibility::Private),
            country_code,
            crs_srid,
        })
    }

    type Error = ProjectValidationError;
}

#[derive(Default, Serialize, Deserialize)]
pub struct PatchProjectPayload {
    pub status: Option<Status>,
    pub name: Option<String>,
    pub visibility: Option<Visibility>,
    pub country_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub crs_srid: Option<Option<i32>>,
    pub slug: Option<String>,
}

impl PatchProjectPayload {
    pub fn try_into_dto(self, project_id: ProjectId) -> Result<ProjectUpdateDto, ApiError> {
        let PatchProjectPayload {
            status,
            name,
            visibility,
            country_code,
            crs_srid,
            slug,
        } = self;

        let name = name
            .map(|n| {
                ProjectNameInputDTO::parse(n).map_err(ProjectValidationError::InvalidProjectName)
            })
            .transpose()?;

        let slug = slug
            .map(|s| {
                ProjectSlugInputDto::try_from(s).map_err(ProjectValidationError::InvalidProjectSlug)
            })
            .transpose()?;

        let country_code = country_code
            .map(|c| {
                isocountry::CountryCode::for_alpha2(&c)
                    .map_err(ProjectValidationError::InvalidCountryCode)
            })
            .transpose()?;

        Ok(ProjectUpdateDto {
            id: project_id,
            status,
            name,
            visibility,
            country_code,
            crs_srid,
            slug,
        })
    }
}
