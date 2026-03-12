use chrono::Datelike;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::features::figure_tool::ids::{BaseMapId, DataProviderId};

use super::BaseMapDataSource;

#[derive(Deserialize, Serialize, Debug, Clone, FromRow)]
pub struct BaseMapOutputDTO {
    pub id: BaseMapId,
    #[sqlx(flatten)]
    pub data_provider: BaseMapDataProvider,
    pub name: String,
    pub slug: String,
    pub default_main_map_base_map: bool,
    pub default_overview_map_base_map: bool,
    pub datasource: Option<sqlx::types::Json<BaseMapDataSource>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BaseMapDataProvider {
    pub id: DataProviderId,
    pub name: String,
    pub copyright_text: Option<String>,
}

impl FromRow<'_, PgRow> for BaseMapDataProvider {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let copyright_text: Option<String> = row
            .try_get::<Option<String>, _>("copyright_text")?
            .map(|text| text.replace("<year>", &chrono::Utc::now().year().to_string()));
        Ok(BaseMapDataProvider {
            id: row.try_get("dp_id")?,
            name: row.try_get("dp_name")?,
            copyright_text,
        })
    }
}

impl BaseMapOutputDTO {
    pub fn overview_map_slug(&self) -> String {
        format!("{}-overview", self.slug)
    }
    pub fn set_url_to_alt_url(&mut self) {
        if let Some(ref mut ds) = self.datasource {
            ds.0.set_url_to_alt_url();
        }
    }
}
