use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

use crate::layer::{PgDataSource, XYZDataSource, datasource::wms::WMSDataSource};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub enum Geometry {
    Polygon,
    Line,
    Point,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Clone)]
pub enum DataType {
    vector,
    raster,
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

#[allow(non_camel_case_types)]
#[derive(Serialize, Default, Clone)]
pub enum DataProvider {
    #[default]
    postgres,
    wms,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Default, Clone)]
pub enum ResourceType {
    #[default]
    dataset,
}

#[derive(Clone)]
pub enum DataSource {
    Postgres(PgDataSource),
    WMS(WMSDataSource),

    XYZ(XYZDataSource),
}

impl Serialize for DataSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            DataSource::Postgres(pg) => pg.to_string(),
            DataSource::WMS(wms) => wms.to_string(),

            DataSource::XYZ(xyz) => xyz.to_string(),
        };
        serializer.serialize_str(&s)
    }
}

impl Default for DataSource {
    fn default() -> Self {
        DataSource::XYZ(XYZDataSource::default())
    }
}

impl From<&WkbType> for Geometry {
    fn from(value: &WkbType) -> Self {
        match value {
            WkbType::MultiPolygon | WkbType::Polygon => Self::Polygon,
            WkbType::LineString | WkbType::MultiLineString => Self::Line,
            WkbType::Point | WkbType::MultiPoint => Self::Point,
        }
    }
}
