use ogcapi_types::common::{
    Link,
    link_rel::{COLLECTION, SELF},
    media_type::{GEO_JSON, JSON},
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

fn deserialize_foreign_members<'de, D>(
    deserializer: D,
) -> Result<Option<Map<String, Value>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: Option<Map<String, Value>> = Option::deserialize(deserializer)?;
    Ok(map.filter(|m| !m.is_empty()))
}

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub enum Type {
    #[default]
    Feature,
}
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feature {
    pub id: i32,
    pub r#type: Type,
    pub properties: Map<String, Value>,
    pub geometry: Option<geojson::Geometry>,
    pub links: [Link; 2],
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_foreign_members",
        flatten,
        default
    )]
    pub foreign_members: Option<serde_json::Map<String, Value>>,
}

impl Feature {
    pub fn new(
        id: i32,
        properties: Map<String, Value>,
        geometry: Option<geojson::Geometry>,
        collection_url: String,
    ) -> Self {
        let links = [
            Link::new(format!("{collection_url}/items/{id}"), SELF).mediatype(GEO_JSON),
            Link::new(collection_url, COLLECTION).mediatype(JSON),
        ];
        Self {
            id,
            r#type: Default::default(),
            properties,
            geometry,
            links,
            foreign_members: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn feature_serialises_and_deserialises_to_and_from_geojson() {
        let extra_field_name = "extra_field";
        let mut fm = serde_json::Map::new();
        fm.insert(extra_field_name.to_string(), json!("some text"));
        let feature = Feature {
            id: 0,
            r#type: Default::default(),
            properties: Default::default(),
            geometry: None,
            links: [Link::new("href", "rel"), Link::new("href", "rel")],
            foreign_members: Some(fm),
        };
        let json = json!(feature);
        let geojson: geojson::Feature =
            serde_json::from_value(json).expect("failed to deserialise to geojson feature");

        // Asserts a GeoJson feature matches required criteria
        // Verify the feature has two links
        let foreign_members = geojson
            .foreign_members
            .as_ref()
            .expect("no foreign members");
        let links = foreign_members.get("links").expect("feature has no links");

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
        let _text = foreign_members.get(extra_field_name).expect(&format!(
            "feature has no '{}' in foreign memeber",
            extra_field_name
        ));

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
        let json = json!(geojson);
        let feature: Feature =
            serde_json::from_value(json).expect("fialed to deserialise to ogc feature");
        assert!(feature.foreign_members.is_some())
    }
}
