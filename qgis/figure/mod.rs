use anyhow::Context;

pub mod spec;
pub use spec::PrintResolution;

use crate::{
    config::QgisFigureConfig,
    figure::spec::{QgisBasemapDataSource, QgisFigureSpec},
    layer::{DataSource, PgConfig, QgisMapLayerBuilder, SslMode, WMSDataSource, XYZDataSource},
    project::{ProjectRoot, QgisProject, QgisProjectBuilder},
    srs::SpatialRefSys,
    enums::EPSGID,
};

mod figure_builder;
mod pg_vector_layer;
use figure_builder::FigureBuilder;
use pg_vector_layer::generate_pg_vector_layer;

#[derive(Debug, Clone)]
pub struct QgisDbConnection {
    pub host: String,
    pub dbname: String,
    pub port: u16,
    pub sslmode: SslMode,
    pub authcfg: Option<String>,
}

pub fn generate_project(
    fig: QgisFigureSpec,
    figure_config: Option<&QgisFigureConfig>,
    print_resolution: &PrintResolution,
    include_figure_id: bool,
    db: PgConfig,
    authcfg: Option<String>,
) -> Result<QgisProject, anyhow::Error> {
    let mut qgis_layers = Vec::new();
    for layer in &fig.layers {
        let qgis_layer = generate_pg_vector_layer(layer, authcfg.clone(), db.clone());
        if let Some(mut l) = qgis_layer {
            l.labelsEnabled = Some(layer.enable_labels as u8);
            qgis_layers.push(l)
        }
    }

    let layout = FigureBuilder::new(&fig, print_resolution, figure_config, &qgis_layers)
        .build(include_figure_id)
        .context("failed to generate builder")?;
    let low_res = match print_resolution {
        PrintResolution::Low => true,
        PrintResolution::High => false,
    };
    let project_name = fig.qgis_project_name(print_resolution).0;

    let mut root = ProjectRoot::new(EPSGID::BNG, fig.map_extent);

    if let Some(map) = fig.basemap
        && let Some(datasource) = map.datasource
    {
        let layer = QgisMapLayerBuilder::try_from((map.slug, datasource))?
            .build_raster(fig.properties.greyscale_background_map.unwrap_or(false))?;
        qgis_layers.push(layer)
    }

    if let Some(map) = fig.overview_basemap {
        let layer_name = map.overview_map_slug();
        if let Some(datasource) = map.datasource {
            let mut builder = QgisMapLayerBuilder::try_from((layer_name.clone(), datasource))?;
            builder.layer_name = layer_name;
            let layer = builder
                .build_raster(false)
                .context("failed to build raster base map layer")?;
            qgis_layers.push(layer)
        }
    }

    root.add_layout(layout);
    for layer in qgis_layers {
        root.add_layer(layer);
    }

    let qgis_project_builder = QgisProjectBuilder {
        project_name,
        root,
        figure_id: fig.id,
        low_res,
    };

    let qgis_project = qgis_project_builder
        .build_with_layer_styles(fig.layers)
        .context("failed to create qgis project")?;

    Ok(qgis_project)
}

impl TryFrom<(String, QgisBasemapDataSource)> for QgisMapLayerBuilder {
    type Error = anyhow::Error;

    fn try_from((layer_slug, src): (String, QgisBasemapDataSource)) -> Result<Self, Self::Error> {
        let epsg_id = match &src {
            QgisBasemapDataSource::XYZ { .. } => 3857u16,
            QgisBasemapDataSource::WMS { epsg_id, .. } => *epsg_id,
            QgisBasemapDataSource::WMTS { epsg_id, .. } => *epsg_id,
        };
        let srs = match epsg_id {
            27700 => SpatialRefSys::bng(),
            4326 => SpatialRefSys::wgs84(),
            3857 => SpatialRefSys::web_mercator(),
            _ => {
                return Err(anyhow::anyhow!(
                    "unsupported coordinate reference system for base map: {}. Currently supported CRS are epsg 27700, 4326 and 3857",
                    epsg_id
                ));
            }
        };
        let ds = match src {
            QgisBasemapDataSource::XYZ { url } => DataSource::XYZ(XYZDataSource { url }),
            QgisBasemapDataSource::WMTS {
                authcfg_id,
                url,
                layers,
                epsg_id,
                tile_matrix_set,
            } => DataSource::WMS(WMSDataSource::new_wmts(
                authcfg_id,
                url,
                layers,
                epsg_id,
                tile_matrix_set,
            )),
            QgisBasemapDataSource::WMS {
                authcfg_id,
                url,
                layers,
                epsg_id,
            } => DataSource::WMS(WMSDataSource::new_wms(authcfg_id, url, layers, epsg_id)),
        };
        Ok(QgisMapLayerBuilder {
            layer_name: layer_slug,
            legend_text: None,
            include_on_legend: false,
            datasource: ds,
            srs: Some(srs),
        })
    }
}
