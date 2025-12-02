use crate::enums::Visibility;

pub struct ProjectInputDto {
    pub name: String,
    pub visibility: Visibility,
    pub country_code: isocountry::CountryCode,
    pub crs_srid: Option<i32>,
}
