use std::str::FromStr;

use domain::ProjectId;
use qgis::layer::{Geometry, WkbType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize)]
pub struct ProjectLayerOutputDTO {
    pub table_name: String,
    pub schema_name: String,
    pub epsg_id: i32,
    pub wkb_type: WkbType,
    pub geometry_type: Geometry,
    pub layer_name: String,
    pub project_id: ProjectId,
    pub owner: String,
}

impl FromRow<'_, sqlx::postgres::PgRow> for ProjectLayerOutputDTO {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        let table_name: String = row.try_get("table_name")?;
        let schema_name: String = row.try_get("schemaname")?;
        let epsg_id: i32 = row.try_get("epsg_id")?;
        let wkb_type = WkbType::from_str(&row.try_get::<String, _>("geometry_type")?)
            .map_err(|e| sqlx::Error::Decode(format!("Invalid geometry type: {}", e).into()))?;
        let owner: String = row.try_get("owner")?;

        // Parse project_id from table name prefix (e.g., "p00001_test" -> 1, "p1 test" -> 1)
        let table_name_lower = table_name.to_lowercase();
        let after_p = table_name_lower.strip_prefix('p').ok_or_else(|| {
            sqlx::Error::Decode(format!("Table name must start with 'p': {}", table_name).into())
        })?;

        // Split on first space or underscore to get the project number
        let project_id_str = after_p.split(['_', ' ']).next().ok_or_else(|| {
            sqlx::Error::Decode(format!("Invalid prefix name format: {}", table_name).into())
        })?;

        let project_id = project_id_str.parse::<i32>().map_err(|_| {
            sqlx::Error::Decode(format!("Invalid project ID number: {}", table_name).into())
        })?;

        // Extract layer name by removing prefix (e.g., "p00001_test_polygon" -> "test_polygon", "P1 test" -> "test")
        let layer_name = table_name_lower
            .find(['_', ' '])
            .and_then(|idx| {
                if idx + 1 < table_name.len() {
                    Some(table_name[idx + 1..].to_string())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                sqlx::Error::Decode(format!("Invalid table name format: {}", table_name).into())
            })?;

        Ok(Self {
            table_name,
            schema_name,
            epsg_id,
            geometry_type: (&wkb_type).into(),
            wkb_type,
            layer_name,
            project_id: ProjectId(project_id),
            owner,
        })
    }
}
