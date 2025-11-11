use crate::{
    ogc::types::common::{
        Link,
        link_relations::{COLLECTION, SELF},
        media_types::MediaType,
    },
    repo::ogc::FeatureRow,
};
use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Serialize, Default)]
pub enum Type {
    #[default]
    Feature,
}

/// Abstraction of real world phenomena (ISO 19101-1:2014)
#[derive(Serialize)]
pub struct Feature {
    pub id: i32,
    pub r#type: Type,
    pub properties: Map<String, Value>,
    pub geometry: geojson::Geometry,
    pub links: FeatureLinks,
}

pub struct FeatureLinks {
    pub to_self: Link,
    pub to_collection: Link,
}

impl Serialize for FeatureLinks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self {
            to_self,
            to_collection,
        } = &self;
        vec![to_self, to_collection].serialize(serializer)
    }
}

impl Feature {
    pub fn from_feature_row(row: FeatureRow, collection_url: String) -> Self {
        let FeatureRow {
            id,
            properties,
            geometry,
        } = row;
        Self {
            id,
            r#type: Type::default(),
            properties,
            geometry,
            links: FeatureLinks {
                to_self: Link::new(format!("{collection_url}/items/{id}"), SELF)
                    .mediatype(MediaType::GeoJson),
                to_collection: Link::new(collection_url, COLLECTION).mediatype(MediaType::Json),
            },
        }
    }
}
