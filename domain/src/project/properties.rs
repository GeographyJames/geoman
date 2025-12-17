use crate::{
    AddedBy, LastUpdatedBy, Owner,
    enums::{Status, Visibility},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Properties {
    pub name: String,
    pub added: DateTime<Utc>,
    #[serde(flatten)]
    pub owner: Owner,
    #[serde(flatten)]
    pub added_by: AddedBy,
    pub technologies: Vec<String>,
    pub country_code: String, // ISO 3166-1-ALPHA-2
    pub subdivisions: Vec<String>,
    pub status: Status,
    pub visibility: Visibility,
    pub crs_srid: Option<i32>,
    #[serde(flatten)]
    pub last_updated_by: LastUpdatedBy,
    pub last_updated: DateTime<Utc>,
    pub slug: String,
    pub search_area_id: Option<i32>,
    pub search_site_name: Option<String>,
}
