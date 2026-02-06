use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{Team, UserId};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, FromRow, Default)]
pub struct User {
    pub id: UserId,
    pub first_name: String,
    pub last_name: String,
    pub clerk_id: Option<String>,
    pub team: Team,
    pub operating_country_code: Option<String>,
    pub admin: bool,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug, Default)]
#[sqlx(type_name = "app.user")]
pub struct AddedBy {
    #[serde(rename = "added_by_id")]
    id: UserId,
    #[serde(rename = "added_by_first_name")]
    first_name: String,
    #[serde(rename = "added_by_last_name")]
    last_name: String,
    #[serde(skip_serializing)]
    clerk_id: Option<String>,
    #[serde(rename = "added_by_team")]
    team: Team,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug, Default)]
#[sqlx(type_name = "app.user")]
pub struct Owner {
    #[serde(rename = "owner_id")]
    id: UserId,
    #[serde(rename = "owner_first_name")]
    first_name: String,
    #[serde(rename = "owner_last_name")]
    last_name: String,
    #[serde(skip_serializing)]
    clerk_id: Option<String>,
    #[serde(rename = "owner_team")]
    team: Team,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug, Default)]
#[sqlx(type_name = "app.user")]
pub struct LastUpdatedBy {
    #[serde(rename = "last_updated_by_id")]
    id: UserId,
    #[serde(rename = "last_updated_by_first_name")]
    first_name: String,
    #[serde(rename = "last_updated_by_last_name")]
    last_name: String,
    #[serde(skip_serializing)]
    clerk_id: Option<String>,
    #[serde(rename = "last_updated_by_team")]
    team: Team,
}
