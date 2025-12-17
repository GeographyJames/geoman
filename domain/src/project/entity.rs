use crate::{IntoOGCFeature, ProjectId, project::Properties};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value, json};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub id: ProjectId,
    pub properties: Properties,
    pub centroid: Option<geojson::Geometry>,
}

impl IntoOGCFeature for Project {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let Project {
            id,
            properties,
            centroid,
        } = self;
        let properties: Map<String, Value> = from_value(json!(properties)).unwrap();

        ogc::Feature::new(id.0, properties, centroid, collection_url)
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
            id: ProjectId(id),
            properties,
            centroid: geometry,
        })
    }
}
