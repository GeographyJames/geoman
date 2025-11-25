use serde_json::Value;

use crate::IntoOGCFeature;

pub struct Feature {
    pub id: i32,

    pub geom: geojson::Geometry,
    pub properties: serde_json::Map<String, Value>,
}

impl IntoOGCFeature for Feature {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature
where {
        let Feature {
            id,
            geom,
            properties,
        } = self;
        ogc::Feature::new(id, properties, Some(geom), collection_url)
    }
}
