use serde::Deserialize;

use crate::Password;

#[derive(Deserialize, Clone)]
pub struct GeoserverSettings {
    pub username: String,
    pub password: Password,
    pub url: String,
}
