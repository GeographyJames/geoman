use crate::{AddedBy, IntoOGCFeature, LastUpdatedBy, enums::Status};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value, json};
use sqlx::prelude::FromRow;

pub struct TurbineLayout {
    pub id: i32,
    pub properties_map: Map<String, Value>,
    pub properties: Properties,
    pub geometry: geojson::Geometry,
}

#[derive(Serialize, Deserialize, Default, FromRow)]
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
    pub rotor_diameter_mm: Option<i32>,
    pub hub_height_mm: Option<i32>,
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
