use crate::IntoOGCFeature;
use anyhow::{Context, anyhow};
use serde_json::{Map, Value, json};

pub struct ProjectFeature {
    pub id: i32,
    pub collection_id: i32,
    pub project_id: i32,
    pub properties: Map<String, Value>,
    pub name: String,
    pub storage_crs_srid: i32,
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
            collection_id,
            project_id,
            storage_crs_srid,
            ..
        } = self;
        ogc::Feature::new(id, collection_url)
            .set_geometry(geometry)
            .set_properties(properties)
            .insert_property("name".to_string(), json!(name))
            .insert_property("is_primary".to_string(), json!(is_primary))
            .insert_property("collection_id".to_string(), json!(collection_id))
            .insert_property("project_id".to_string(), json!(project_id))
            .insert_property("storage_crs_srid".to_string(), json!(storage_crs_srid))
    }
}

impl TryFrom<ogc::Feature> for ProjectFeature {
    type Error = anyhow::Error;
    fn try_from(mut ogc_feature: ogc::Feature) -> Result<Self, Self::Error> {
        let name = ogc_feature
            .remove_string_property("name")
            .context("No 'name' field in feature properties")??;
        let ogc::Feature {
            id,
            mut properties,
            geometry,
            ..
        } = ogc_feature;
        let is_primary: bool = serde_json::from_value(
            properties
                .remove("is_primary")
                .ok_or_else(|| anyhow!("No 'is_primary' field in feature properties"))?,
        )?;
        let collection_id: i32 = serde_json::from_value(
            properties
                .remove("collection_id")
                .ok_or_else(|| anyhow!("No 'collection_id' field in feature properties"))?,
        )?;
        let project_id: i32 = serde_json::from_value(
            properties
                .remove("project_id")
                .ok_or_else(|| anyhow!("No 'project_id' field in feature properties"))?,
        )?;
        let storage_crs_srid: i32 = serde_json::from_value(
            properties
                .remove("storage_crs_srid")
                .ok_or_else(|| anyhow!("No 'project_id' field in feature properties"))?,
        )?;
        Ok(Self {
            id,
            collection_id,
            properties,
            name,
            geometry: geometry.ok_or(anyhow!("feature has no geometry"))?,
            is_primary,
            project_id,
            storage_crs_srid,
        })
    }
}
