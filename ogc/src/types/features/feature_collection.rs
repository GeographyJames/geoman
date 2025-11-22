use crate::types::Feature;
use ogcapi_types::common::{Link, link_rel::SELF, media_type::GEO_JSON};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize)]
pub enum Type {
    #[default]
    FeatureCollection,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureCollection {
    pub id: String,
    pub r#type: Type,
    pub features: Vec<Feature>,
    pub links: [Link; 1],
    pub time_stamp: String,
    pub number_matched: i64,
    pub number_returned: usize,
}

impl FeatureCollection {
    pub fn new(
        collection_url: &str,
        collection_id: String,
        features: Vec<Feature>,
        number_matched: i64,
    ) -> Self {
        Self {
            id: collection_id,
            r#type: Type::default(),
            number_returned: features.len(),
            features,
            links: Self::links(collection_url),
            //todo set timestamp from database
            time_stamp: chrono::Utc::now().to_rfc3339(),
            number_matched,
        }
    }
    fn links(collection_url: &str) -> [Link; 1] {
        [Link::new(format!("{}/items", collection_url), SELF).mediatype(GEO_JSON)]
    }

    pub fn opening_json(
        collection_id: &str,
        collection_url: &str,
        number_matched: i64,
    ) -> Result<String, serde_json::Error> {
        let mut json = format!(
            r#"{{"type":{},"id":{},"links":{},"timeStamp":{},"numberMatched":{}"#,
            serde_json::to_string(&Type::default())?,
            serde_json::to_string(collection_id)?,
            serde_json::to_string(&Self::links(collection_url))?,
            serde_json::to_string(&chrono::Utc::now().to_rfc3339())?,
            serde_json::to_string(&number_matched)?
        );

        json.push_str(r#","features":["#);
        Ok(json)
    }
    pub fn closing_json(number_returned: usize) -> Result<String, serde_json::Error> {
        Ok(format!(
            r#"],"numberReturned":{}}}"#,
            serde_json::to_string(&number_returned)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    impl Default for FeatureCollection {
        fn default() -> Self {
            Self::new(
                &uuid::Uuid::new_v4().to_string(),
                uuid::Uuid::new_v4().to_string(),
                Vec::new(),
                0,
            )
        }
    }

    #[test]
    fn feature_collection_serialises_to_and_from_geojson() {
        let feature_collection = FeatureCollection::default();
        let json = json!(feature_collection);
        let geojson: geojson::FeatureCollection = serde_json::from_value(json)
            .expect("failed to deserialize to geojson feature collection");
        check_geojson(&geojson);
        let json = json!(geojson);
        let _feature_collection: FeatureCollection =
            serde_json::from_value(json).expect("failed to deserialise to ogc feature collection");
    }

    #[test]
    fn feature_collection_opening_and_closing_json_serialises_to_geojson_and_deserialises_to_feature_collection()
     {
        let json_string = format!(
            "{}{}",
            FeatureCollection::opening_json("0", "0", 0).expect("failed to serialise opening json"),
            FeatureCollection::closing_json(0).expect("failed to serialise closing json")
        );
        let _: FeatureCollection = serde_json::from_str(&json_string)
            .expect("failed to deserialise to feature collection");
    }

    fn check_geojson(geojson: &geojson::FeatureCollection) {
        let foreign_members = geojson
            .foreign_members
            .as_ref()
            .expect("collection has no foreign members");

        // Verifyt collection has a link
        let links = foreign_members
            .get("links")
            .expect("collection has no links");
        match links {
            serde_json::Value::Array(values) => {
                let links: Vec<Link> = values
                    .into_iter()
                    .map(|v| {
                        serde_json::from_value(v.clone()).expect("failed to deserialise links")
                    })
                    .collect();
                assert_eq!(links.len(), 1, "collection should have 1 link")
            }
            _ => panic!("links is not an array"),
        }

        // Verify collection has a string id
        let id = foreign_members.get("id").expect("collection has no id");
        match id {
            serde_json::Value::String(_) => {}
            _ => panic!("id is not a string"),
        }
    }
}
