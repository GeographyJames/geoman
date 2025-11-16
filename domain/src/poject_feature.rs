use crate::IntoOGCFeature;
use anyhow::anyhow;
use serde_json::{Map, Value, json};

pub struct ProjectFeature {
    pub id: i32,
    pub properties: Map<String, Value>,
    pub name: String,
    pub geometry: geojson::Geometry,
    pub is_primary: bool,
}

impl IntoOGCFeature for ProjectFeature {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let ProjectFeature {
            id,
            properties,
            geometry,
            name,
            is_primary,
        } = self;
        ogc::Feature::new(id, collection_url)
            .set_geometry(geometry)
            .set_properties(properties)
            .insert_property("name".to_string(), json!(name))
            .insert_property("is_primary".to_string(), json!(is_primary))
    }
}

impl TryFrom<ogc::Feature> for ProjectFeature {
    type Error = anyhow::Error;
    fn try_from(value: ogc::Feature) -> Result<Self, Self::Error> {
        let ogc::Feature {
            id,
            mut properties,
            geometry,
            ..
        } = value;
        let name: String = serde_json::from_value(
            properties
                .remove("name")
                .ok_or(anyhow!("No 'name' field in feature properties"))?,
        )?;
        let is_primary: bool = serde_json::from_value(
            properties
                .remove("is_primary")
                .ok_or(anyhow!("No 'is_primary' field in feature properties"))?,
        )?;
        Ok(Self {
            id,
            properties,
            name,
            geometry: geometry.ok_or(anyhow!("feature has no geometry"))?,
            is_primary,
        })
    }
}
