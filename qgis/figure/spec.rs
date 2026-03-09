use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Extent,
    enums::ScalebarUnits,
    layer::{PgConfig, WkbType},
    layout::{PageSize, Size, components::SizeInteger},
};

#[derive(Clone, Copy)]
pub enum PrintResolution {
    High = 300,
    Low = 96,
}

pub struct QgisProjectName(pub String);

// ---------------------------------------------------------------------------
// Basemap
// ---------------------------------------------------------------------------

pub struct QgisDataProvider {
    pub copyright_text: Option<String>,
}

pub struct QgisBasemapSpec {
    pub slug: String,
    pub datasource: Option<QgisBasemapDataSource>,
    pub data_provider: QgisDataProvider,
}

impl QgisBasemapSpec {
    pub fn overview_map_slug(&self) -> String {
        format!("{}-overview", self.slug)
    }
}

pub enum QgisBasemapDataSource {
    XYZ {
        url: String,
    },
    WMS {
        url: String,
        layers: String,
        authcfg_id: Option<String>,
        epsg_id: u16,
    },
    WMTS {
        url: String,
        layers: String,
        authcfg_id: Option<String>,
        epsg_id: u16,
        tile_matrix_set: String,
    },
}

// ---------------------------------------------------------------------------
// Layers
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum SupportedEpsg {
    BNG = 27700,
    WGS84 = 4326,
}

impl From<SupportedEpsg> for u16 {
    fn from(value: SupportedEpsg) -> Self {
        value as u16
    }
}

pub enum QgisProjectLayer {
    Valid {
        schema: String,
        table: String,
        wkb_type: WkbType,
        epsg_id: SupportedEpsg,
    },
    Invalid(String),
}

pub enum QgisLayerSource {
    SiteBoundary { id: i32 },
    TurbineLayout { id: i32 },
    ProjectLayer(QgisProjectLayer),
}

pub struct QgisLayerSpec {
    pub name: String,
    pub styleqml: Option<String>,
    pub source: QgisLayerSource,
    pub legend_text: Option<String>,
    pub include_on_legend: bool,
    pub include_on_map: bool,
    pub include_as_target: bool,
    pub enable_labels: bool,
    pub convert_boundary_to_singleparts: bool,
}

// ---------------------------------------------------------------------------
// Figure properties
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum CopyrightText {
    Default,
    Custom,
    #[default]
    None,
}

#[derive(Default, Clone)]
pub struct QgisFigureProperties {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub extra_legend_text: Option<String>,
    pub enable_html: Option<bool>,
    pub figure_number: Option<String>,
    pub scalebar_units: Option<ScalebarUnits>,
    pub scalebar_units_per_segment: Option<u32>,
    pub logo: Option<bool>,
    pub internal_use: Option<bool>,
    pub map_ticks: Option<bool>,
    pub north_arrow: Option<bool>,
    pub status: Option<String>,
    pub scalebar: Option<bool>,
    pub copyright_text: Option<CopyrightText>,
    pub custom_copyright_text: Option<String>,
    pub greyscale_background_map: Option<bool>,
    pub legend_text_vmargin: Option<u32>,
    pub legend: Option<bool>,
    pub grid_spacing: Option<u32>,
    pub legend_height_percent: Option<u32>,
    pub overview_frame: Option<bool>,
    pub overview_map_scale: Option<u32>,
}

// ---------------------------------------------------------------------------
// Figure spec — the qgis crate's input type replacing FigureOutputDTO
// ---------------------------------------------------------------------------

pub struct QgisFigureSpec {
    pub id: i32,
    pub project_id: i32,
    pub project_name: String,
    pub qgis_project_uuid: uuid::Uuid,
    pub page_width_mm: i32,
    pub page_height_mm: i32,
    pub margin_mm: i32,
    pub legend_width_mm: i32,
    pub scale: i32,
    pub map_extent: Extent,
    pub overview_map_extent: Extent,
    pub properties: QgisFigureProperties,
    pub layers: Vec<QgisLayerSpec>,
    pub basemap: Option<QgisBasemapSpec>,
    pub overview_basemap: Option<QgisBasemapSpec>,
    // For ID stamp (user_id_with_initials_and_last_updated)
    pub last_updated_by_id: i32,
    pub last_updated_by_first_name: String,
    pub last_updated_by_last_name: String,
    pub last_updated: DateTime<Utc>,
}

impl QgisFigureSpec {
    pub fn page_size(&self) -> Size {
        Size {
            width_mm: self.page_width_mm as f64,
            height_mm: self.page_height_mm as f64,
        }
    }

    pub fn map_right(&self) -> u32 {
        if self.properties.legend_height_percent.unwrap_or(100) < 100 {
            (self.page_width_mm - self.margin_mm) as u32
        } else {
            (self.page_width_mm - self.margin_mm - self.legend_width_mm) as u32
        }
    }

    pub fn layout_name(&self) -> String {
        let mut name =
            String::from(self.properties.title.as_deref().unwrap_or("untitled"));
        if let Some(subtitle) = &self.properties.subtitle {
            name.push_str(&format!("-{}", subtitle));
        }
        slug::slugify(name)
    }

    pub fn qgis_project_name(&self, resolution: &PrintResolution) -> QgisProjectName {
        match resolution {
            PrintResolution::High => QgisProjectName(self.qgis_project_uuid.to_string()),
            PrintResolution::Low => {
                QgisProjectName(format!("{}_low-res", self.qgis_project_uuid))
            }
        }
    }

    pub fn filename(&self) -> String {
        let mut filename =
            String::from(self.properties.title.as_deref().unwrap_or("untitled"));
        if let Some(n) = self.properties.figure_number.as_deref() {
            filename.push('_');
            filename.push_str(n);
        }
        if let Some(s) = self.properties.subtitle.as_deref() {
            filename.push('_');
            filename.push_str(s);
        }
        slug::slugify(filename)
    }

    pub fn filename_with_id(&self, suffix: &str) -> String {
        format!("{}_{:05}.{}", self.filename(), self.id, suffix)
    }

    pub fn filename_without_id(&self, suffix: &str) -> String {
        format!("{}.{}", self.filename(), suffix)
    }

    pub fn page_name(&self) -> String {
        PageSize::from_dimensions(SizeInteger {
            width_mm: self.page_width_mm as u32,
            height_mm: self.page_height_mm as u32,
        })
        .map(|p| p.name())
        .unwrap_or_else(|| {
            format!(
                "paper dimensions w: {}mm, h: {}mm",
                self.page_width_mm, self.page_height_mm
            )
        })
    }

    pub fn unique_boundary_ids_on_map(&self) -> HashSet<i32> {
        self.layers
            .iter()
            .filter(|l| l.include_on_map)
            .filter_map(|l| {
                if let QgisLayerSource::SiteBoundary { id } = l.source {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn unique_layout_ids_on_map(&self) -> HashSet<i32> {
        self.layers
            .iter()
            .filter(|l| l.include_on_map)
            .filter_map(|l| {
                if let QgisLayerSource::TurbineLayout { id } = l.source {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn user_id_with_initials_and_last_updated(&self) -> String {
        let mut user_id = format!("U{:04}", self.last_updated_by_id);
        if let (Some(first), Some(last)) = (
            self.last_updated_by_first_name.chars().next(),
            self.last_updated_by_last_name.chars().next(),
        ) {
            user_id.push(first);
            user_id.push(last);
        }
        user_id.push(' ');
        user_id.push_str(&format!(
            "{}",
            self.last_updated.format("%d%m%y %H:%M:%S%Z")
        ));
        user_id
    }

    pub fn map_layer_names(&self) -> Vec<String> {
        self.layers
            .iter()
            .filter(|l| l.include_on_map)
            .filter(|l| !matches!(l.source, QgisLayerSource::ProjectLayer(QgisProjectLayer::Invalid(_))))
            .map(|l| l.name.clone())
            .collect()
    }
}
