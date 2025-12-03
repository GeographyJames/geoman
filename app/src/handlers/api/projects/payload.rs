use anyhow::Context;
use domain::{
    enums::Visibility,
    project::{ProjectInputDto, ProjectSlug},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectReqPayload {
    pub name: String,
    pub visibility: Visibility,
    pub country_code: String,
    pub crs_srid: Option<i32>,
}

impl ProjectReqPayload {
    pub fn try_into_dto(self) -> Result<ProjectInputDto, anyhow::Error> {
        let ProjectReqPayload {
            name,
            visibility,
            country_code,
            crs_srid,
        } = self;
        Ok(ProjectInputDto {
            slug: ProjectSlug::parse(&name),
            name,
            visibility,
            country_code: isocountry::CountryCode::for_alpha2(&country_code)
                .context("failed to parse country code")?,
            crs_srid,
        })
    }
}
