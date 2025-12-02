use crate::{IntoOGCFeature, ProjectId, project::Properties};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value, json};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub id: ProjectId,
    pub properties: Properties,
    pub centroid: Option<geojson::Geometry>,
    pub centroid_in_storage_crs: Option<geojson::Geometry>,
}

impl IntoOGCFeature for Project {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature {
        let Project {
            id,
            properties,
            centroid,
            centroid_in_storage_crs,
        } = self;
        let properties: Map<String, Value> = from_value(json!(properties)).unwrap();

        let mut ft = ogc::Feature::new(id.0, properties, centroid, collection_url);
        if let Some(centroid) = centroid_in_storage_crs {
            let mut foreign_members = Map::new();
            foreign_members.insert("centroid_in_storage_crs".to_string(), json!(centroid));
            ft.foreign_members = Some(foreign_members);
        };
        ft
    }
}

impl TryFrom<ogc::Feature> for Project {
    type Error = anyhow::Error;

    fn try_from(ogc_feature: ogc::Feature) -> Result<Self, Self::Error> {
        let ogc::Feature {
            id,
            properties,
            geometry,
            foreign_members,
            ..
        } = ogc_feature;
        let properties = serde_json::from_value(Value::Object(properties))?;

        let centroid_in_storage_crs = if let Some(mut fm) = foreign_members {
            let g = fm.remove("centroid_in_storage_crs");
            let geo: Result<Option<geojson::Geometry>, _> =
                g.map(serde_json::from_value).transpose();
            geo?
        } else {
            None
        };

        Ok(Project {
            id: ProjectId(id),
            properties,
            centroid: geometry,
            centroid_in_storage_crs,
        })
    }
}
