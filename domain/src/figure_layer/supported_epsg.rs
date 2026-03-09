use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Copy)]
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
