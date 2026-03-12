use serde::Serialize;

use crate::qgis::{
    Extent, LayerTreeGroup,
    enums::EPSGID,
    layer::MapLayer,
    layout::{QgisLayout, QgisLayoutBuilder},
    project::{MapCanvas, ProjectProperties},
    srs::Srs,
};

#[derive(Serialize)]
#[serde(rename = "qgis")]
pub struct ProjectRoot {
    #[serde(rename = "@projectname")]
    pub projectname: String,
    #[serde(rename = "@version")]
    pub version: String,
    pub title: String,
    #[serde(rename = "projectCrs")]
    pub project_crs: Srs,
    #[serde(rename = "mapcanvas")]
    pub map_canvases: Vec<MapCanvas>,
    #[serde(rename = "Layouts")]
    pub layouts: Option<Layouts>,
    pub properties: ProjectProperties,
    #[serde(rename = "projectlayers")]
    pub project_layers: ProjectLayers,
    #[serde(rename = "layer-tree-group")]
    pub layer_tree_group: LayerTreeGroup,
}

#[derive(Serialize, Default)]
pub struct ProjectLayers {
    #[serde(rename = "maplayer")]
    pub map_layers: Vec<MapLayer>,
}

impl ProjectRoot {
    pub fn add_layout(&mut self, layout: QgisLayoutBuilder) {
        match &mut self.layouts {
            None => {
                self.layouts = Some(Layouts {
                    layouts: vec![layout.build()],
                })
            }
            Some(layouts) => layouts.layouts.push(layout.build()),
        }
    }
    pub fn add_layer(&mut self, layer: MapLayer) {
        self.layer_tree_group.add_layer(
            layer.id.clone(),
            layer.layername.clone(),
            layer.datasource.clone(),
            layer.legend_text.clone(),
            layer.provider.text.clone(),
        );

        self.project_layers.map_layers.push(layer);
    }
}

#[derive(Serialize)]
pub struct Layouts {
    #[serde(rename = "Layout")]
    layouts: Vec<QgisLayout>,
}

impl ProjectRoot {
    pub fn new(project_srs_id: EPSGID, extent: Extent) -> ProjectRoot {
        Self {
            projectname: Default::default(),
            version: "3.34.4-Prizren".to_string(),
            title: Default::default(),
            map_canvases: vec![MapCanvas::main(project_srs_id, extent)],
            layouts: Default::default(),
            project_crs: Srs {
                spatialrefsys: project_srs_id.qgis_srs(),
            },
            properties: Default::default(),
            project_layers: Default::default(),
            layer_tree_group: Default::default(),
        }
    }
    pub fn new_wgs84() -> ProjectRoot {
        let crs = EPSGID::WGS84;
        ProjectRoot::new(crs, crs.default_extent())
    }
    pub fn new_bng() -> ProjectRoot {
        let crs = EPSGID::BNG;
        ProjectRoot::new(crs, crs.default_extent())
    }
}
