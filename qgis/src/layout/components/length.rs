use serde::Serialize;

use crate::enums::Units;

#[derive(Default)]
pub struct Length {
    pub distance: f32,
    pub units: Units,
}

impl Serialize for Length {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let formatted = format!("{},{}", self.distance, self.units);
        serializer.serialize_str(&formatted)
    }
}
