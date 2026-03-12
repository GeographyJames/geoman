use serde::Serialize;

#[derive(Serialize, Default)]
pub struct ProjectProperties {
    #[serde(rename = "SpatialRefSys")]
    spatial_ref_sys: PropertiesSrs,
}

#[derive(Serialize, Default)]
struct PropertiesSrs {
    #[serde(rename = "ProjectionsEnabled")]
    projections_enabled: ProjectionsEnabled,
}

#[derive(Serialize)]
struct ProjectionsEnabled {
    #[serde(rename = "@type")]
    srs_type: String,
    #[serde(rename = "$text")]
    value: u8,
}

impl Default for ProjectionsEnabled {
    fn default() -> Self {
        Self {
            srs_type: "int".into(),
            value: 1,
        }
    }
}
