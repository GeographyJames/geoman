use crate::IntoOGCFeature;

use ogcapi_types::common::{
    Link,
    link_rel::{COLLECTION, SELF},
    media_type::{GEO_JSON, JSON},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value, json};

#[derive(Serialize, Deserialize)]
pub struct Properties {
    pub slug: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Project {
    pub id: i32,
    pub properties: Properties,
}

impl IntoOGCFeature for Project {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let Project { id, properties } = self;
        let properties: Map<String, Value> = from_value(json!(properties)).unwrap();
        let links = [
            Link::new(format!("{collection_url}/items/{id}"), SELF).mediatype(GEO_JSON),
            Link::new(&collection_url, COLLECTION).mediatype(JSON),
        ];
        ogc::Feature {
            id,
            r#type: Default::default(),
            properties,
            geometry: None,
            links,
        }
    }
}

impl TryFrom<ogc::Feature> for Project {
    type Error = anyhow::Error;

    fn try_from(ogc_feature: ogc::Feature) -> Result<Self, Self::Error> {
        let ogc::Feature { id, properties, .. } = ogc_feature;
        let properties = serde_json::from_value(Value::Object(properties))?;

        Ok(Project { id, properties })
    }
}
