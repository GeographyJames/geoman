use serde::Deserialize;
use strum::{Display, EnumString};

/// Available runtime environments
#[derive(Deserialize, EnumString, Display, Clone)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GeoManEnvironment {
    Local,
    Demo,
    Staging,
    Production,
    CiPipeline,
}
