mod spatial_ref_system;
pub use spatial_ref_system::{SpatialRefSys, Srs};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum SupportedEpsg {
    BNG = 27700,
    WGS84 = 4326,
}

impl Serialize for SupportedEpsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for SupportedEpsg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match i32::deserialize(deserializer)? {
            27700 => Ok(SupportedEpsg::BNG),
            4326 => Ok(SupportedEpsg::WGS84),
            other => Err(serde::de::Error::custom(format!(
                "unknown EPSG id: {}",
                other
            ))),
        }
    }
}
