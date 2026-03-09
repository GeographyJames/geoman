use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone, Copy)]
pub struct Extent {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

impl Extent {
    pub fn uk() -> Self {
        Self {
            xmin: 0.,
            ymin: 0.,
            xmax: 700_000.,
            ymax: 1_300_000.,
        }
    }
    pub fn wgs84() -> Self {
        Self {
            xmin: -180.,
            ymin: -90.,
            xmax: 180.,
            ymax: 90.,
        }
    }
}

impl Display for Extent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.xmin, self.ymin, self.xmax, self.ymax)
    }
}
#[derive(Serialize)]
pub struct LayoutMapExtent {
    #[serde(rename = "@xmax")]
    pub xmax: f64,
    #[serde(rename = "@xmin")]
    pub xmin: f64,
    #[serde(rename = "@ymax")]
    pub ymax: f64,
    #[serde(rename = "@ymin")]
    pub ymin: f64,
}

impl Default for LayoutMapExtent {
    fn default() -> Self {
        Self {
            xmax: 180.,
            xmin: -180.,
            ymax: 90.,
            ymin: -90.,
        }
    }
}

impl From<Extent> for LayoutMapExtent {
    fn from(value: Extent) -> Self {
        let Extent {
            xmax,
            xmin,
            ymax,
            ymin,
        } = value;
        Self {
            xmax,
            xmin,
            ymax,
            ymin,
        }
    }
}
