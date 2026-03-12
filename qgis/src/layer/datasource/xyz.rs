use std::fmt::Display;

#[derive(Clone, Default)]
pub struct XYZDataSource {
    pub url: String,
}

impl Display for XYZDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let processed_url = if self.url.contains("?") {
            let parts: Vec<&str> = self.url.splitn(2, '?').collect();
            let base_url = parts[0];
            let query_params = parts.get(1).unwrap_or(&"");
            let encoded_params = query_params
                .replace("=", "%3D")
                .replace("&", "%26")
                .replace("{", "%7B")
                .replace("}", "%7D");
            format!("{}?{}", base_url, encoded_params)
        } else {
            // If no query params, just encode the placeholder variables
            self.url
                .replace("=", "%3D")
                .replace("&", "%26")
                .replace("{", "%7B")
                .replace("}", "%7D")
        };

        write!(f, "type=xyz&url={}", processed_url)
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::layer::{DataSource, datasource::xyz::XYZDataSource};
    #[derive(Serialize)]
    struct Root {
        datasource: DataSource,
    }

    #[test]
    fn wms_datasource_to_string_works() {
        let ds = Root {
            datasource: DataSource::XYZ(XYZDataSource {
                url: "https://tile.openstreetmap.org/{z}/{x}/{y}.png".into(),
            }),
        };

        let s = quick_xml::se::to_string(&ds).expect("failed to create xml");
        assert_eq!(
            &s,
            "<Root><datasource>type=xyz&amp;url=https://tile.openstreetmap.org/%7Bz%7D/%7Bx%7D/%7By%7D.png</datasource></Root>"
        );
        let ds = Root {
            datasource: DataSource::XYZ(XYZDataSource {
                url: "https://mt0.google.com/vt/lyrs=y&x={x}&y={y}&z={z}".into(),
            }),
        };
        let s = quick_xml::se::to_string(&ds).expect("failed to create xml");

        assert_eq!(
            &s,
            "<Root><datasource>type=xyz&amp;url=https://mt0.google.com/vt/lyrs%3Dy%26x%3D%7Bx%7D%26y%3D%7By%7D%26z%3D%7Bz%7D</datasource></Root>"
        );
    }
}
