use crate::{AddedBy, IntoOGCFeature, LastUpdatedBy, enums::Status};
use chrono::{DateTime, Utc};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
use serde_json::{Map, Value, from_value, json};
use std::fmt;

pub struct TurbineLayout {
    pub id: i32,
    pub properties_map: Map<String, Value>,
    pub properties: Properties,
    pub geometry: geojson::Geometry,
}

pub enum TurbineMeasurement {
    None,
    Various,
    SingleValue(i32),
}

impl Default for TurbineMeasurement {
    fn default() -> Self {
        Self::None
    }
}

impl Serialize for TurbineMeasurement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => s.serialize_none(),
            Self::Various => s.serialize_str("various"),
            Self::SingleValue(v) => s.serialize_i32(*v),
        }
    }
}

impl<'de> Deserialize<'de> for TurbineMeasurement {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(TurbineMeasurementVisitor)
    }
}

struct TurbineMeasurementVisitor;

impl<'de> Visitor<'de> for TurbineMeasurementVisitor {
    type Value = TurbineMeasurement;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("null, \"various\", or an integer")
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(TurbineMeasurement::None)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(TurbineMeasurement::None)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "various" => Ok(TurbineMeasurement::Various),
            _ => Err(de::Error::unknown_variant(v, &["various"])),
        }
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(TurbineMeasurement::SingleValue(v as i32))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(TurbineMeasurement::SingleValue(v as i32))
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Properties {
    pub collection_id: i32,
    pub project_id: i32,
    pub id: i32,
    pub name: String,
    pub storage_crs_srid: i32,
    pub is_primary: bool,
    pub status: Status,
    pub added: DateTime<Utc>,
    #[serde(flatten)]
    pub added_by: AddedBy,
    pub last_updated: DateTime<Utc>,
    #[serde(flatten)]
    pub last_updated_by: LastUpdatedBy,
    pub collection_title: String,
    pub rotor_diameter_mm: TurbineMeasurement,
    pub hub_height_mm: TurbineMeasurement,
    pub turbine_count: i64,
}

impl IntoOGCFeature for TurbineLayout {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let TurbineLayout {
            id,
            properties,
            geometry,
            mut properties_map,
            ..
        } = self;
        let mut additional: Map<String, Value> = from_value(json!(properties)).unwrap();
        properties_map.append(&mut additional);
        ogc::Feature::new(id, properties_map, Some(geometry), collection_url)
    }
}
