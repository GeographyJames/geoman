use serde::Serialize;

pub struct QgisUuid(pub uuid::Uuid);

impl Default for QgisUuid {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Serialize for QgisUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{{{}}}", self.0))
    }
}
