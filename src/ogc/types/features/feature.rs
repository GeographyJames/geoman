use crate::{
    ogc::types::common::{
        Link,
        link_relations::{COLLECTION, SELF},
        media_types::MediaType,
    },
    postgres::ogc::features::FeatureRow,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Serialize, Default, Deserialize)]
pub enum Type {
    #[default]
    Feature,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct Feature {
    pub id: i32,
    pub r#type: Type,
    pub properties: Map<String, Value>,
    pub geometry: geojson::Geometry,
    pub links: [Link; 2],
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
            links: [
                Link::new(format!("{collection_url}/items/{id}"), SELF)
                    .mediatype(MediaType::GeoJson),
                Link::new(collection_url, COLLECTION).mediatype(MediaType::Json),
            ],
        }
    }
}

impl Default for Feature {
    fn default() -> Self {
        let row = FeatureRow {
            id: Default::default(),
            properties: Default::default(),
            geometry: geojson::Geometry::new(geojson::Value::Point(vec![1., 1.])),
        };
        Self::from_feature_row(row, Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::ogc::{
        self,
        types::{Feature, common::Link},
    };
    use serde_json::json;

    #[test]
    fn feature_serialises_and_deserialises_to_and_from_geojson() {
        let feature = Feature::default();
        let json = json!(feature);
        let geojson: geojson::Feature =
            serde_json::from_value(json).expect("failed to deserialise to geojson feature");
        check_geojson(&geojson);
        let json = json!(geojson);
        let _feature: ogc::types::Feature =
            serde_json::from_value(json).expect("fialed to deserialise to ogc feature");
    }

    /// Asserts a GeoJson feature matches required criteria
    fn check_geojson(geojson: &geojson::Feature) {
        // Verify the feature has geometry
        assert!(geojson.geometry.is_some(), "feature has no geometry");

        // Verify the feature has two links
        let links = geojson
            .foreign_members
            .as_ref()
            .expect("no foreign members")
            .get("links")
            .expect("feature has no links");
        match links {
            serde_json::Value::Array(values) => {
                let links: Vec<Link> = values
                    .into_iter()
                    .map(|v| {
                        serde_json::from_value(v.clone()).expect("failed to deserialise links")
                    })
                    .collect();
                assert_eq!(links.len(), 2, "feature should have 2 links")
            }
            _ => panic!("links is not an array"),
        }

        // Verify the feature has integer id
        let id = geojson.id.as_ref().expect("feature has no id");

        match id {
            geojson::feature::Id::Number(number) => {
                let _id_value: i32 = number
                    .as_i64()
                    .expect("feature id is not a valid i64")
                    .try_into()
                    .expect("feature id is not valid i32");
            }
            geojson::feature::Id::String(_) => panic!("feature id not a number"),
        }
    }
}
