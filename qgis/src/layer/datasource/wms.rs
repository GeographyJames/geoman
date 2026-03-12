use std::fmt::Display;

#[derive(Clone)]
pub struct WMSDataSource {
    authcfg: Option<String>,
    epsg_code: u16,
    dpi_mode: u16,
    format: String,
    layers: String,
    url: String,
    tile_pixel_ratio: u16,
    styles: Option<String>,
    wmts_tile_matrix_set: Option<String>,
}

impl WMSDataSource {
    pub fn new_wms(authcfg: Option<String>, url: String, layers: String, epsg_code: u16) -> Self {
        Self {
            authcfg,
            epsg_code,
            dpi_mode: 7,
            format: "image/png".to_string(),
            layers,
            url,
            tile_pixel_ratio: 0,
            styles: None,
            wmts_tile_matrix_set: None,
        }
    }

    pub fn new_wmts(
        authcfg: Option<String>,
        url: String,
        layers: String,
        epsg_code: u16,
        wmts_tile_matrix_set: String,
    ) -> Self {
        Self {
            authcfg,
            epsg_code,
            dpi_mode: 7,
            format: "image/png".to_string(),
            layers,
            url,
            tile_pixel_ratio: 0,
            styles: Some("raster".into()),
            wmts_tile_matrix_set: Some(wmts_tile_matrix_set),
        }
    }
}

impl Display for WMSDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();

        if let Some(authcfg) = &self.authcfg {
            parts.push(format!("authcfg={}", authcfg));
        }

        parts.push(format!("crs=EPSG:{}", self.epsg_code));
        parts.push(format!("dpiMode={}", self.dpi_mode));
        parts.push(format!("format={}", self.format));
        parts.push(format!("layers={}", self.layers));

        if let Some(ref styles) = self.styles {
            parts.push(format!("styles={}", styles))
        } else {
            parts.push("styles".into())
        }
        if let Some(ref tile_matrix) = self.wmts_tile_matrix_set {
            parts.push(format!("tileMatrixSet={}", tile_matrix))
        }
        parts.push(format!("tilePixelRatio={}", self.tile_pixel_ratio));
        parts.push(format!("url={}", self.url));

        write!(f, "{}", parts.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::qgis::layer::{DataSource, datasource::wms::WMSDataSource};

    #[derive(Serialize)]
    struct Root {
        datasource: DataSource,
    }

    #[test]
    fn to_string_works_for_os_50k_wms() {
        let ds = Root {
            datasource: DataSource::WMS(WMSDataSource::new_wms(
                Some("y1mj99p".to_string()),
                "https://geoserver.geodata-manager.com/geoserver/wms".to_string(),
                "osdata:50k".to_string(),
                27700,
            )),
        };
        let s = quick_xml::se::to_string(&ds).expect("failed to create xml");
        assert_eq!(
            &s,
            "<Root><datasource>authcfg=y1mj99p&amp;crs=EPSG:27700&amp;dpiMode=7&amp;format=image/png&amp;layers=osdata:50k&amp;styles&amp;tilePixelRatio=0&amp;url=https://geoserver.geodata-manager.com/geoserver/wms</datasource></Root>"
        )
    }
}
