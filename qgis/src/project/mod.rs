mod root;
pub use root::ProjectRoot;
mod map_canvas;
pub use map_canvas::MapCanvas;

mod properties;
pub use properties::ProjectProperties;

use std::io::{Cursor, Write};

use anyhow::Context;
use chrono::Utc;

use serde::{Deserialize, Serialize};
use zip::{ZipWriter, result::ZipError, write::FileOptions};

use crate::helpers::insert_renderer_v2_into_project;
use crate::helpers::unzip_content;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QgisProjectMetadata {
    last_modified_time: chrono::DateTime<Utc>,
    last_modified_user: String,
}

/// Minimal layer info needed to apply style XML when building a QGIS project.
pub struct QgisLayerStyle {
    pub name: String,
    pub styleqml: Option<String>,
}

#[derive(Debug)]
pub struct QgisProject {
    pub name: String,
    pub metadata: sqlx::types::Json<QgisProjectMetadata>,
    pub content: Vec<u8>,
    pub figure_id: i32,
    pub low_res: bool,
}

impl QgisProject {
    pub fn content_to_xml_string(&self) -> Result<String, ZipError> {
        let content_string = unzip_content(&self.content)?;
        Ok(content_string)
    }
}

pub struct QgisProjectBuilder {
    pub project_name: String,
    pub figure_id: i32,
    pub low_res: bool,
    pub root: ProjectRoot,
}

impl QgisProjectBuilder {
    pub fn build(self) -> Result<QgisProject, anyhow::Error> {
        let xml = format!(
            "<!DOCTYPE qgis PUBLIC 'http://mrcc.com/qgis.dtd' 'SYSTEM'>{}",
            quick_xml::se::to_string(&self.root).context("failed to serialize QgisRoot to xml")?
        );
        let content = create_zip_in_memory(&xml, &format!("{}.qgs", self.project_name))
            .context("failed to zip content")?;

        let project = QgisProject {
            name: self.project_name,
            metadata: Default::default(),
            content,
            low_res: self.low_res,
            figure_id: self.figure_id,
        };
        Ok(project)
    }
    pub fn build_with_layer_styles(
        self,
        layers: Option<Vec<QgisLayerStyle>>,
    ) -> Result<QgisProject, anyhow::Error> {
        use crate::helpers::extract_all_style_elements;

        let mut xml =
            quick_xml::se::to_string(&self.root).context("failed to serialize QgisRoot to xml")?;

        // Insert each layer style
        if let Some(layers) = layers {
            for layer in layers {
                if let Some(styleqml) = layer.styleqml {
                    let style_elements = extract_all_style_elements(&styleqml).map_err(|e| {
                        anyhow::anyhow!(
                            "failed to extract style elements for layer '{}': {}",
                            layer.name,
                            e
                        )
                    })?;

                    xml = insert_renderer_v2_into_project(&xml, &style_elements, &layer.name)
                        .map_err(|e| {
                            anyhow::anyhow!(
                                "failed to insert style for layer '{}': {}",
                                layer.name,
                                e
                            )
                        })?;
                }
            }
        }

        let xml = format!(
            "<!DOCTYPE qgis PUBLIC 'http://mrcc.com/qgis.dtd' 'SYSTEM'>{}",
            xml
        );

        let content = create_zip_in_memory(&xml, &format!("{}.qgs", self.project_name))
            .context("failed to zip content")?;

        let project = QgisProject {
            name: self.project_name,
            metadata: Default::default(),
            content,
            low_res: self.low_res,
            figure_id: self.figure_id,
        };
        Ok(project)
    }
}

fn create_zip_in_memory(content: &str, filename: &str) -> zip::result::ZipResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let cursor = Cursor::new(&mut buffer);
    let mut zip = ZipWriter::new(cursor);

    zip.start_file(filename, FileOptions::<()>::default())?;
    zip.write_all(content.as_bytes())?;
    zip.finish()?;

    Ok(buffer)
}

impl Default for QgisProjectMetadata {
    fn default() -> Self {
        Self {
            last_modified_time: Utc::now(),
            last_modified_user: "app".to_string(),
        }
    }
}
