use anyhow::Context;

mod figure_builder;
mod pg_vector_layer;

use crate::{
    app::{
        configuration::QgisFigureConfig,
        features::figure_tool::dtos::{
            base_map::BaseMapDataSource,
            figure::FigureOutputDTO,
        },
    },
    qgis::{
        enums::EPSGID,
        layer::{DataSource, PgConfig, QgisMapLayerBuilder, WMSDataSource, XYZDataSource},
        project::{ProjectRoot, QgisLayerStyle, QgisProject, QgisProjectBuilder},
        srs::SpatialRefSys,
    },
};

use figure_builder::FigureBuilder;
use pg_vector_layer::generate_pg_vector_layer;

#[derive(Copy, Clone)]
pub enum PrintResolution {
    High = 300,
    Low = 96,
}

pub fn generate_project(
    fig: FigureOutputDTO,
    figure_config: Option<&QgisFigureConfig>,
    print_resolution: &PrintResolution,
    include_figure_id: bool,
    db: PgConfig,
    authcfg: Option<String>,
) -> Result<QgisProject, anyhow::Error> {
    let mut qgis_layers = Vec::new();
    if let Some(layers) = (fig.layers).as_ref() {
        for layer in layers {
            let qgis_layer = generate_pg_vector_layer(layer, authcfg.clone(), db.clone());
            if let Some(mut l) = qgis_layer {
                l.labelsEnabled = Some(layer.properties.enable_labels as u8);
                qgis_layers.push(l)
            }
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

    if let Some(map) = fig.main_map_base_map
        && let Some(datasource) = map.datasource
    {
        let layer = QgisMapLayerBuilder::try_from((map.slug, datasource.0))?
            .build_raster(fig.properties.greyscale_background_map.unwrap_or(false))?;
        qgis_layers.push(layer)
    }

    if let Some(map) = fig.overview_map_base_map {
        let layer_name = map.overview_map_slug();
        if let Some(datasource) = map.datasource {
            let mut builder = QgisMapLayerBuilder::try_from((layer_name.clone(), datasource.0))?;
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

    let layer_styles: Option<Vec<QgisLayerStyle>> = fig.layers.as_ref().map(|layers| {
        layers
            .iter()
            .map(|l| QgisLayerStyle {
                name: l.name.clone(),
                styleqml: l.styleqml.clone(),
            })
            .collect()
    });

    let qgis_project_builder = QgisProjectBuilder {
        project_name,
        root,
        figure_id: fig.id.0,
        low_res,
    };

    let qgis_project = qgis_project_builder
        .build_with_layer_styles(layer_styles)
        .context("failed to create qgis project")?;

    Ok(qgis_project)
}

impl TryFrom<(String, BaseMapDataSource)> for QgisMapLayerBuilder {
    type Error = anyhow::Error;

    fn try_from((layer_slug, src): (String, BaseMapDataSource)) -> Result<Self, Self::Error> {
        let epsg_id = src.epsg_id();
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
            BaseMapDataSource::XYZ(ds) => DataSource::XYZ(XYZDataSource { url: ds.url }),
            BaseMapDataSource::WMTS(ds) => DataSource::WMS(WMSDataSource::new_wmts(
                ds.authcfg_id,
                ds.url,
                ds.layers,
                ds.epsg_id,
                ds.tile_matrix_set,
            )),
            BaseMapDataSource::WMS(ds) => DataSource::WMS(WMSDataSource::new_wms(
                ds.authcfg_id,
                ds.url,
                ds.layers,
                ds.epsg_id,
            )),
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
