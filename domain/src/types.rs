use ogcapi_types::common::Crs;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Default, Debug)]
pub struct ProjectFeatureId {
    pub collection_id: ProjectCollectionId,
    pub id: i32,
}
#[derive(Clone, Copy, Default, Debug, Deserialize)]
pub struct ProjectId(pub i32);
#[derive(Clone, Copy, Default, Debug)]
pub struct UserId(pub i32);
#[derive(Clone, Copy, Default, Debug)]
pub struct TeamId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize)]
pub struct FeatureId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct KeyId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize)]
pub struct ProjectCollectionId(pub i32);

impl Display for ProjectCollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for ProjectFeatureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "collection id: {}, feature id: {}",
            self.collection_id, self.id
        )
    }
}

impl Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for FeatureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Clone)]
pub struct SupportedCrs(Vec<Crs>);

impl SupportedCrs {
    pub fn new(storage_crs: Option<Crs>) -> Self {
        let default_crs = Crs::default();
        let crs = if let Some(ref crs) = storage_crs
            && default_crs.as_srid() != crs.as_srid()
        {
            vec![default_crs, crs.clone()]
        } else {
            vec![default_crs]
        };
        Self(crs)
    }
    pub fn into_inner(self) -> Vec<Crs> {
        self.0
    }
}
