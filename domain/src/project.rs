use ogc::types::Feature;
use serde::Deserialize;
use serde_json::json;

use crate::IntoOGCFeature;

#[derive(Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl IntoOGCFeature for Project {
    fn into_ogc_feature(self, collection_url: String) -> ogc::types::Feature {
        let Project { id, name, .. } = self;
        let mut project = Feature::new(id, collection_url);
        project.properties.insert("name".to_string(), json!(name));
        project
    }
}
