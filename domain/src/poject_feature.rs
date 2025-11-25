use crate::IntoOGCFeature;
use anyhow::{Context, anyhow};
use ogcapi_types::common::Crs;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value, json};

pub struct ProjectFeature {
    pub id: i32,
    pub properties_map: Map<String, Value>,
    pub properties: Properties,
    pub geometry: geojson::Geometry,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Properties {
    pub collection_id: i32,
    pub project_id: i32,
    pub name: String,
    pub storage_crs: Crs,
    pub is_primary: bool,
}

#[derive(Deserialize)]
struct PropertiesHelper {
    #[serde(flatten)]
    properties: Properties,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl IntoOGCFeature for ProjectFeature {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let ProjectFeature {
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

impl TryFrom<ogc::Feature> for ProjectFeature {
    type Error = anyhow::Error;
    fn try_from(ogc_feature: ogc::Feature) -> Result<Self, Self::Error> {
        let ogc::Feature {
            id,
            properties,
            geometry,
            ..
        } = ogc_feature;

        let helper: PropertiesHelper = serde_json::from_value(Value::Object(properties))
            .context("Failed to deserialize properties")?;
        Ok(Self {
            id,
            properties: helper.properties,
            properties_map: helper.extra,
            geometry: geometry.ok_or(anyhow!("feature has no geometry"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntoOGCFeature, ProjectFeature};
    use serde_json::{Map, json};

    #[test]
    fn feature_converts_to_and_from_ogc_feature() {
        let mut map = Map::new();
        let key = uuid::Uuid::new_v4().to_string();
        map.insert(key.clone(), json!(uuid::Uuid::new_v4().to_string()));
        let ft = ProjectFeature {
            id: 1,
            properties_map: map,
            properties: Default::default(),
            geometry: geojson::Geometry::new(geojson::Value::Point(vec![1., 1.])),
        };
        let ogc = ft.into_ogc_feature(uuid::Uuid::new_v4().to_string());
        let ft = ProjectFeature::try_from(ogc).unwrap();
        assert!(ft.properties_map.contains_key(&key));
    }
}
