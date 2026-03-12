use serde::Serialize;

#[derive(Default, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let formatted = format!("{},{},mm", self.x, self.y);
        serializer.serialize_str(&formatted)
    }
}
