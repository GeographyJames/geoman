use crate::{AddedBy, IntoOGCFeature, LastUpdatedBy, enums::Status};
use anyhow::{Context, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value, json};
use sqlx::prelude::FromRow;

pub struct ProjectFeature {
    pub id: i32,
    pub properties_map: Map<String, Value>,
    pub properties: Properties,
    pub geometry: geojson::Geometry,
}

#[derive(Serialize, Deserialize, Default, FromRow)]
pub struct Properties {
    pub collection_id: i32,
    pub project_id: i32,
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
            mut properties,
            geometry,
            ..
        } = ogc_feature;

        // Extract system properties
        let properties_struct: Properties =
            serde_json::from_value(Value::Object(properties.clone()))
                .context("Failed to deserialize system properties")?;

        // Remove all known system fields from properties map to leave only user-defined fields
        // This includes base fields and flattened added_by/last_updated_by fields
        let system_fields = [
            "collection_id",
            "project_id",
            "name",
            "storage_crs_srid",
            "is_primary",
            "status",
            "added",
            "last_updated",
            "added_by_id",
            "added_by_first_name",
            "added_by_last_name",
            "added_by_team",
            "last_updated_by_id",
            "last_updated_by_first_name",
            "last_updated_by_last_name",
            "last_updated_by_team",
            "collection_title",
        ];

        for field in system_fields {
            properties.remove(field);
        }

        Ok(Self {
            id,
            properties: properties_struct,
            properties_map: properties,
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
