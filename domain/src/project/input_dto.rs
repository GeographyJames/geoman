use crate::{enums::Visibility, project::ProjectSlug};

pub struct ProjectInputDto {
    pub name: String,
    pub visibility: Visibility,
    pub country_code: isocountry::CountryCode,
    pub crs_srid: Option<i32>,
    pub slug: ProjectSlug,
}
