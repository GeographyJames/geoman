use serde::Serialize;

use crate::domain::enums::SupportedEpsg;
#[derive(Serialize, Clone)]
#[serde(rename = "spatialrefsys")]
pub struct SpatialRefSys {
    proj4: String,
    srsid: u32,
    srid: u32,
    authid: String,
    description: String,
    projectionacronym: String,
    ellipsoidacronym: String,
    geographicflag: bool,
}

impl SpatialRefSys {
    pub fn bng() -> Self {
        Self {
            proj4: "+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +units=m +no_defs".into(),
            srsid: 2437,
            srid: 27700,
            authid: "EPSG:27700".into(),
            description: "OSGB36 / British National Grid".into(),
            projectionacronym: "tmerc".into(),
            ellipsoidacronym: "EPSG:7001".into(),
            geographicflag: false,
        }
    }

    pub fn wgs84() -> Self {
        Self {
            proj4: "+proj=longlat +datum=WGS84 +no_defs".to_string(),
            srsid: 3452,
            srid: 4326,
            authid: "EPSG:4326".into(),
            description: "WGS 84".into(),
            projectionacronym: "longlat".into(),
            ellipsoidacronym: "EPSG:7030".into(),
            geographicflag: true,
        }
    }

    pub fn web_mercator() -> Self {
        Self { proj4: "+proj=merc +a=6378137 +b=6378137 +lat_ts=0 +lon_0=0 +x_0=0 +y_0=0 +k=1 +units=m +nadgrids=@null +wktext +no_defs".into(), srsid: 3857, srid: 3857, authid: "EPSG:3857".into(), description:  "WGS 84 / Pseudo-Mercator".into(), projectionacronym: "merc".into(), ellipsoidacronym: "EPSG:7030".into(), geographicflag: false }
    }
    pub fn from_epsg(epsg_id: i32) -> Result<Self, String> {
        match epsg_id {
            27700 => Ok(Self::bng()),
            4326 => Ok(Self::wgs84()),
            _ => Err(format!(
                "unsuported epsg id: {epsg_id}, currently only supported codes are 27700 and 4326"
            )),
        }
    }
}
#[derive(Serialize, Clone)]
pub struct Srs {
    pub spatialrefsys: SpatialRefSys,
}

impl From<SupportedEpsg> for SpatialRefSys {
    fn from(value: SupportedEpsg) -> Self {
        match value {
            SupportedEpsg::BNG => SpatialRefSys::bng(),
            SupportedEpsg::WGS84 => SpatialRefSys::wgs84(),
        }
    }
}
