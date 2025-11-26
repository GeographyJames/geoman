use crate::IntoOGCFeature;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value, json};

#[derive(Serialize, Deserialize)]
pub struct Properties {
    pub name: String,
    pub centroid_in_storage_crs: Option<geojson::Geometry>,
}

pub struct ProjectName(pub String);

#[derive(Deserialize)]
pub struct Project {
    pub id: i32,
    pub properties: Properties,
    pub geom: Option<geojson::Geometry>,
}

impl IntoOGCFeature for Project {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let Project {
            id,
            properties,
            geom,
        } = self;
        let properties: Map<String, Value> = from_value(json!(properties)).unwrap();
        ogc::Feature::new(id, properties, geom, collection_url)
    }
}

impl TryFrom<ogc::Feature> for Project {
    type Error = anyhow::Error;

    fn try_from(ogc_feature: ogc::Feature) -> Result<Self, Self::Error> {
        let ogc::Feature {
            id,
            properties,
            geometry,
            ..
        } = ogc_feature;
        let properties = serde_json::from_value(Value::Object(properties))?;

        Ok(Project {
            id,
            properties,
            geom: geometry,
        })
    }
}
