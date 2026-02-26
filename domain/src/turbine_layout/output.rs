use crate::{
    domain::{
        dtos::{Id, UserId},
        enums::DataStatus,
    },
    geo::WkbGeom,
};
use anyhow::{Context, anyhow};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct TurbineLayoutOutputDTO {
    pub id: Id,
    pub project_id: Id,
    pub name: String,
    pub primary: bool,
    pub status: DataStatus,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_id: UserId,
    pub added: chrono::DateTime<Utc>,
    #[serde(skip_serializing)]
    pub turbines: Vec<TurbineOutputDTO>,
}

#[derive(Serialize)]
pub struct TurbineOutputDTO {
    #[serde(skip_serializing)]
    pub id: Id,
    pub layout_id: Id,
    pub turbine_number: i32,
    pub blade_length_mm: Option<i32>,
    pub hub_height_mm: Option<i32>,
    pub easting_bng: i32,
    pub northing_bng: i32,
    #[serde(skip_serializing)]
    pub geom: WkbGeom,
}

impl TryInto<geojson::FeatureCollection> for TurbineLayoutOutputDTO {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<geojson::FeatureCollection, Self::Error> {
        Ok(geojson::FeatureCollection {
            foreign_members: Some(
                json!(self)
                    .as_object()
                    .ok_or_else(|| anyhow!("failed to create properties json map"))?
                    .to_owned(),
            ),
            features: self
                .turbines
                .into_iter()
                .map(|t| t.try_into().context("failed to create turbine geom"))
                .collect::<Result<Vec<geojson::Feature>, anyhow::Error>>()?,
            bbox: None,
        })
    }
}

impl TryInto<geojson::Feature> for TurbineOutputDTO {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<geojson::Feature, Self::Error> {
        let properties = json!(self)
            .as_object()
            .cloned()
            .ok_or_else(|| anyhow!("failed to create properties json map"))?;
        let geom: geojson::Geometry = self
            .geom
            .try_into()
            .context("failed to convert geometry field to geojson geometry")?;
        Ok(geojson::Feature {
            bbox: None,
            geometry: Some(geom),
            id: Some(self.id.into()),
            properties: Some(properties),
            foreign_members: None,
        })
    }
}

impl std::fmt::Display for TurbineLayoutOutputDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "<TurbineLayoutOutputDTO> id: {}, project_id: {}, name: {}",
            self.id.0, self.project_id.0, self.name
        )
    }
}
