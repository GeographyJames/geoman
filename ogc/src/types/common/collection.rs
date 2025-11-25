// This is a copy of ogcapi-types collection purely for openapi docs

use ogcapi_types::common::{Crs, Extent, Link};
use serde::Serialize;

use serde_with::DisplayFromStr;
use utoipa::ToSchema;

// const CRS_REF: &str = "#/crs";

#[derive(Serialize, Default)]
pub enum Type {
    #[default]
    Collection,
}

/// A body of resources that belong or are used together. An aggregate, set, or group of related resources.
#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub r#type: Type,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    /// Attribution for the collection.
    pub attribution: Option<String>,
    pub extent: Option<Extent>,
    /// An indicator about the type of the items in the collection.
    pub item_type: Option<String>,
    /// The list of coordinate reference systems supported by the API; the first item is the default coordinate reference system.
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[schema(value_type = Vec<String>)]
    pub crs: Vec<Crs>,
    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schema(value_type = String)]
    pub storage_crs: Option<Crs>,
    pub storage_crs_coordinate_epoch: Option<f32>,
    #[serde(default)]
    pub links: Vec<Link>,
}
