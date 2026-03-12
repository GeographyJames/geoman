use serde::Serialize;

use crate::{
    Extent,
    enums::{EPSGID, MapUnits},
    srs::Srs,
};

#[derive(Serialize)]
pub struct MapCanvas {
    units: MapUnits,
    extent: Extent,

    destinationsrs: Srs,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "expressionContextScope")]
    expression_context_scope: String,
    rendermaptile: u16,
    rotation: u16,
    #[serde(rename = "@annotationsVisible")]
    annotations_visible: u16,
}

impl MapCanvas {
    pub fn main(project_srs_id: EPSGID, extent: Extent) -> Self {
        Self {
            units: project_srs_id.map_units(),
            extent,
            destinationsrs: Srs {
                spatialrefsys: project_srs_id.qgis_srs(),
            },
            name: "theMapCanvas".into(),
            expression_context_scope: Default::default(),
            rendermaptile: Default::default(),
            rotation: Default::default(),
            annotations_visible: 1,
        }
    }

    pub fn for_layout(name: &str, project_srs_id: EPSGID) -> Self {
        Self {
            units: project_srs_id.map_units(),
            extent: project_srs_id.default_extent(),
            destinationsrs: Srs {
                spatialrefsys: project_srs_id.qgis_srs(),
            },
            name: name.to_string(),
            expression_context_scope: Default::default(),
            rendermaptile: 0,
            rotation: 0,
            annotations_visible: 1,
        }
    }
}
