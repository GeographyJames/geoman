use serde::Serialize;

use crate::{
    ogc::types::{
        Feature,
        common::{Link, link_relations::SELF, media_types::MediaType},
    },
    repo::ogc::FeatureCollectionRow,
};

#[derive(Serialize, Default)]
pub enum Type {
    #[default]
    FeatureCollection,
}

#[derive(Serialize)]
pub struct FeatureCollection {
    pub id: String,
    pub r#type: Type,
    pub features: Vec<Feature>,
    pub links: FeatureCollectionLinks,
}

pub struct FeatureCollectionLinks {
    pub to_self: Link,
}

impl Serialize for FeatureCollectionLinks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self { to_self } = self;
        vec![to_self].serialize(serializer)
    }
}

impl FeatureCollection {
    pub fn from_feature_collection_row(
        row: FeatureCollectionRow,
        collection_url: String,
        slug: String,
    ) -> Self {
        let FeatureCollectionRow { features } = row;
        Self {
            id: slug,
            r#type: Type::default(),
            features: features
                .into_iter()
                .map(|f| Feature::from_feature_row(f, collection_url.clone()))
                .collect(),
            links: FeatureCollectionLinks {
                to_self: Link::new(format!("{}/items", collection_url), SELF)
                    .mediatype(MediaType::GeoJson),
            },
        }
    }
}
