use crate::IntoOGCFeature;
use anyhow::Context;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl IntoOGCFeature for Project {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let Project { id, name, slug, .. } = self;
        let mut project = ogc::Feature::new(id, collection_url);
        project.properties.insert("name".to_string(), json!(name));
        project.properties.insert("slug".to_string(), json!(slug));
        project
    }
}

impl TryFrom<ogc::Feature> for Project {
    type Error = anyhow::Error;

    fn try_from(mut ogc_feature: ogc::Feature) -> Result<Self, Self::Error> {
        let ogc::Feature { id, .. } = ogc_feature;
        let name = ogc_feature
            .remove_string_property("name")
            .context("feature has no name field")??;
        let slug = ogc_feature
            .remove_string_property("slug")
            .context("feature has no slug feild")??;

        Ok(Project { id, name, slug })
    }
}
