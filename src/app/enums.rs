use serde::Deserialize;
use strum::{Display, EnumString};

/// Available runtime environments
#[derive(Deserialize, EnumString, Display, Clone)]
#[strum(ascii_case_insensitive, serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum GeoManEnvironment {
    Development,
    Demo,
    Staging,
    Production,
    CiPipeline,
}
