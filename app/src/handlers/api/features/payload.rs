use actix_multipart::form::{MultipartForm, tempfile::TempFile};

#[derive(MultipartForm)]
pub struct FeatureInputPayload {
    pub shp: Option<TempFile>,
    pub dbf: Option<TempFile>,
    pub shx: Option<TempFile>,
    pub prj: Option<TempFile>,
    pub shz: Option<TempFile>,
    pub name: actix_multipart::form::text::Text<String>,
    pub primary: Option<actix_multipart::form::text::Text<bool>>,
    pub hub_height_default_metre: Option<actix_multipart::form::text::Text<f64>>,
    pub rotor_diameter_default_metre: Option<actix_multipart::form::text::Text<f64>>,
    pub turbine_number_field: Option<actix_multipart::form::text::Text<String>>,
    pub rotor_diameter_field: Option<actix_multipart::form::text::Text<String>>,
    pub hub_height_field: Option<actix_multipart::form::text::Text<String>>,
}
