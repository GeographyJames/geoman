use crate::figure_layer::SupportedEpsg;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumString;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct PgTableOutputDTO {
    pub table: String,
    pub schema: String,
    pub wkb_type: WkbType,
    pub geometry_type: Geometry,
    pub epsg_id: SupportedEpsg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgTableInvalidOutputDTO {
    pub table: String,
    pub schema: String,
    pub message: String,
}

#[derive(Clone, EnumString, Debug)]
#[strum(ascii_case_insensitive)]
pub enum WkbType {
    MultiPolygon,
    Polygon,
    Point,
    MultiPoint,
    LineString,
    MultiLineString,
}

impl Serialize for WkbType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for WkbType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for WkbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WkbType::Point => "Point",
            WkbType::LineString => "LineString",
            WkbType::Polygon => "Polygon",
            WkbType::MultiPoint => "MultiPoint",
            WkbType::MultiLineString => "MultiLineString",
            WkbType::MultiPolygon => "MultiPolygon",
            // ... etc
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub enum Geometry {
    Polygon,
    Line,
    Point,
}

impl From<&WkbType> for Geometry {
    fn from(wkb: &WkbType) -> Self {
        match wkb {
            WkbType::MultiPolygon | WkbType::Polygon => Geometry::Polygon,
            WkbType::MultiLineString | WkbType::LineString => Geometry::Line,
            WkbType::Point | WkbType::MultiPoint => Geometry::Point,
        }
    }
}
