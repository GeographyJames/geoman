use std::{fmt::Display, str::FromStr};

use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use domain::{
    FigureId,
    bounding_box::{BoundingBox, Point},
    figure::FigureOutputDTO,
    figure_layer::{FigureLayerOutputDTO, FigureLayerDatasourceOutput, ProjectLayer, SiteAssetId},
};
use qgis::{
    Extent,
    config::QgisFigureConfig as QgisConfig,
    figure::{
        PrintResolution,
        generate_project,
        spec::{
            CopyrightText, QgisBasemapSpec, QgisFigureProperties,
            QgisFigureSpec, QgisLayerSource, QgisLayerSpec, QgisProjectLayer, SupportedEpsg,
        },
    },
    layer::{PgConfig, SslMode, WkbType as QgisWkbType},
};
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};

use crate::{
    config::{DatabaseSettings, QgisFigureConfig, QgisServerSettings},
    errors::ApiError,
    postgres::PostgresRepo,
};

// ---------------------------------------------------------------------------
// Format enum
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum FigureFormat {
    pdf,
    jpg,
}

impl Display for FigureFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureFormat::pdf => write!(f, "pdf"),
            FigureFormat::jpg => write!(f, "jpg"),
        }
    }
}

// ---------------------------------------------------------------------------
// QGIS Server GetPrint request builder
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct GetPrintRequest {
    pub service: String,
    pub version: String,
    pub request: String,
    pub crs: String,
    pub format: FigureFormat,
    pub template: String,
    pub map: String,
}

impl Default for GetPrintRequest {
    fn default() -> Self {
        Self {
            service: "WMS".into(),
            version: "1.3.0".into(),
            request: "GetPrint".into(),
            template: "default-layout".into(),
            format: FigureFormat::pdf,
            crs: "EPSG:27700".into(),
            map: String::new(),
        }
    }
}

pub struct GetPrintRequestBuilder {
    pub project_name: String,
    pub template: String,
    pub db_name: String,
}

impl GetPrintRequestBuilder {
    pub fn build(self) -> GetPrintRequest {
        GetPrintRequest {
            // `public` schema — qgis_projects is in the public schema in the new system
            map: format!(
                "postgresql://?dbname={}&schema=public&project={}",
                self.db_name, self.project_name
            ),
            template: self.template,
            ..Default::default()
        }
    }
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

#[get("/{id}/{format}")]
#[tracing::instrument(skip(repo, qgis_settings, db_settings, client))]
pub async fn get_print(
    repo: web::Data<PostgresRepo>,
    path: web::Path<(FigureId, FigureFormat)>,
    qgis_settings: web::Data<QgisServerSettings>,
    client: web::Data<reqwest::Client>,
    db_settings: web::Data<DatabaseSettings>,
) -> Result<HttpResponse, ApiError> {
    let (figure_id, format) = path.into_inner();
    let resolution = match format {
        FigureFormat::jpg => PrintResolution::Low,
        FigureFormat::pdf => PrintResolution::High,
    };

    let figure = repo.get_figure(figure_id).await?;

    let filename = figure_filename_with_id(&figure, &format.to_string());
    let project_name = figure_qgis_project_name(&figure, &resolution);

    // Only generate if a project with this name does not already exist.
    // The insert itself always deletes any old low-res project for this figure
    // (so stale low-res projects are cleaned up whenever a new project is stored).
    let exists = repo
        .qgis_project_exists(&project_name)
        .await
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("failed to check for qgis project: {}", e)))?;

    if !exists {
        let qgis_figure_config = to_qgis_figure_config(&qgis_settings.figure_config);
        let spec = build_figure_spec(figure.clone())
            .map_err(|e| ApiError::Unexpected(e.context("failed to build qgis figure spec")))?;

        let qgis_project = generate_project(
            spec,
            Some(&qgis_figure_config),
            &resolution,
            true,
            PgConfig {
                db_name: db_settings.database_name.clone(),
                port: db_settings.port,
                host: db_settings.host.clone(),
                sslmode: SslMode::from(db_settings.require_ssl),
            },
            None,
        )
        .map_err(|e| ApiError::Unexpected(e.context("failed to generate qgis project")))?;

        repo.insert_qgis_project(&qgis_project)
            .await
            .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("failed to save qgis project: {}", e)))?;
    }

    let request = build_qgis_server_request(
        &project_name,
        &figure,
        &qgis_settings,
        &db_settings,
        &client,
        format,
    );
    let response = request.send().await.map_err(|e| {
        ApiError::Unexpected(anyhow::anyhow!(
            "failed to send GetPrint request to QGIS server: {:?}",
            e
        ))
    })?;

    process_qgis_server_response(response, &filename).await
}

// ---------------------------------------------------------------------------
// FigureOutputDTO → QgisFigureSpec conversion
// ---------------------------------------------------------------------------

fn build_figure_spec(figure: FigureOutputDTO) -> Result<QgisFigureSpec, anyhow::Error> {
    let layers = figure
        .layers
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(convert_layer)
        .collect::<Vec<_>>();

    let target_coord = target_coord(&figure)?;
    let map_extent = map_extent(&figure, target_coord);
    let overview_map_extent = overview_map_extent(&figure, target_coord);

    let properties = convert_properties(&figure.properties);

    // TODO: fetch basemap data from DB and convert to QgisBasemapSpec
    let basemap: Option<QgisBasemapSpec> = None;
    let overview_basemap: Option<QgisBasemapSpec> = None;

    Ok(QgisFigureSpec {
        id: figure.id.0,
        project_id: figure.project_id.0,
        project_name: figure.project_name,
        qgis_project_uuid: figure.qgis_project_uuid,
        page_width_mm: figure.page_width_mm,
        page_height_mm: figure.page_height_mm,
        margin_mm: figure.margin_mm,
        legend_width_mm: figure.legend_width_mm,
        scale: figure.scale,
        map_extent,
        overview_map_extent,
        properties,
        layers,
        basemap,
        overview_basemap,
        last_updated_by_id: figure.last_updated_by.0,
        last_updated_by_first_name: figure.last_updated_by_first_name,
        last_updated_by_last_name: figure.last_updated_by_last_name,
        last_updated: figure.last_updated,
    })
}

/// Compute the target coordinate (centre of target layers) in the figure's SRID.
/// Falls back to the UK centre (324636, 673221) in BNG if no target layers have a bounding box.
fn target_coord(figure: &FigureOutputDTO) -> Result<Point, anyhow::Error> {
    // Honour explicit target coordinates from the properties if set
    if let (Some(x), Some(y)) = (
        figure.properties.target_x_coordinate,
        figure.properties.target_y_coordinate,
    ) {
        return Ok(Point {
            x,
            y,
            srid: figure.srid as u32,
        });
    }

    let layers = figure.layers.as_deref().unwrap_or(&[]);
    let target_bounds = target_layers_bounds(layers)?;

    let point = if let Some(bounds) = target_bounds {
        let in_figure_srid = bounds
            .transform(figure.srid as u32)
            .context("failed to transform target layer bounds to figure CRS")?;
        in_figure_srid.centre()
    } else {
        // Default: UK centre in BNG, transformed if the figure uses a different CRS
        default_uk_centre(figure.srid as u32)?
    };

    Ok(Point {
        x: point.x.round(),
        y: point.y.round(),
        srid: figure.srid as u32,
    })
}

fn target_layers_bounds(
    layers: &[FigureLayerOutputDTO],
) -> Result<Option<BoundingBox>, anyhow::Error> {
    let mut bounds: Option<BoundingBox> = None;
    for layer in layers
        .iter()
        .filter(|l| l.properties.include_as_target)
        .filter_map(|l| l.bounding_box.as_ref())
    {
        bounds = Some(match bounds {
            None => *layer,
            Some(existing) => existing
                .combine(layer)
                .context("failed to combine target layer bounding boxes")?,
        });
    }
    Ok(bounds)
}

fn default_uk_centre(target_srid: u32) -> Result<Point, anyhow::Error> {
    use gdal::spatial_ref::{CoordTransform, SpatialRef};
    let x = 324636.0_f64;
    let y = 673221.0_f64;
    if target_srid == 27700 {
        return Ok(Point { x, y, srid: 27700 });
    }
    let mut geom = gdal::vector::Geometry::from_wkt(&format!("POINT ({} {})", x, y))
        .context("failed to create default UK centre geometry")?;
    let mut source = SpatialRef::from_epsg(27700).context("failed to create BNG SpatialRef")?;
    source.set_axis_mapping_strategy(
        gdal::spatial_ref::AxisMappingStrategy::TraditionalGisOrder,
    );
    let mut target =
        SpatialRef::from_epsg(target_srid).context("failed to create target SpatialRef")?;
    target.set_axis_mapping_strategy(
        gdal::spatial_ref::AxisMappingStrategy::TraditionalGisOrder,
    );
    let transform =
        CoordTransform::new(&source, &target).context("failed to create coordinate transform")?;
    geom.transform_inplace(&transform)
        .context("failed to transform default UK centre")?;
    let (tx, ty, _) = geom.get_point(0);
    Ok(Point {
        x: tx,
        y: ty,
        srid: target_srid,
    })
}

fn map_extent(figure: &FigureOutputDTO, target: Point) -> Extent {
    let map_width_mm = map_width(figure);
    let map_height_mm = map_height(figure);
    extent_from_scale(figure.scale as u32, map_width_mm, map_height_mm, target)
}

fn overview_map_extent(figure: &FigureOutputDTO, target: Point) -> Extent {
    const DEFAULT_OVERVIEW_SCALE: u32 = 1_000_000;
    let overview_scale = figure
        .properties
        .overview_map_scale
        .unwrap_or(DEFAULT_OVERVIEW_SCALE);
    let size_mm = figure.legend_width_mm as u32;
    extent_from_scale(overview_scale, size_mm, size_mm, target)
}

fn map_width(figure: &FigureOutputDTO) -> u32 {
    (map_right(figure) - figure.margin_mm as u32).max(0)
}

fn map_height(figure: &FigureOutputDTO) -> u32 {
    ((figure.page_height_mm - figure.margin_mm) as u32).saturating_sub(figure.margin_mm as u32)
}

fn map_right(figure: &FigureOutputDTO) -> u32 {
    if figure.properties.legend_height_percent.unwrap_or(100) < 100 {
        (figure.page_width_mm - figure.margin_mm) as u32
    } else {
        (figure.page_width_mm - figure.margin_mm - figure.legend_width_mm) as u32
    }
}

fn extent_from_scale(scale: u32, width_mm: u32, height_mm: u32, target: Point) -> Extent {
    let width_m = (width_mm as f64 / 1000.0) * scale as f64;
    let height_m = (height_mm as f64 / 1000.0) * scale as f64;
    let half_w = width_m / 2.0;
    let half_h = height_m / 2.0;
    Extent {
        xmin: target.x - half_w,
        xmax: target.x + half_w,
        ymin: target.y - half_h,
        ymax: target.y + half_h,
    }
}

fn convert_layer(layer: &FigureLayerOutputDTO) -> QgisLayerSpec {
    let source = match &layer.source {
        FigureLayerDatasourceOutput::SiteBoundary(ds) => {
            if let SiteAssetId::BoundryId(id) = ds.id {
                QgisLayerSource::SiteBoundary { id: id.0 }
            } else {
                unreachable!("SiteBoundary datasource must have BoundryId")
            }
        }
        FigureLayerDatasourceOutput::TurbineLayout(ds) => {
            if let SiteAssetId::TurbineLayout(id) = ds.id {
                QgisLayerSource::TurbineLayout { id: id.0 }
            } else {
                unreachable!("TurbineLayout datasource must have TurbineLayout id")
            }
        }
        FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Valid(dto)) => {
            let wkb_type = QgisWkbType::from_str(&dto.wkb_type.to_string())
                .expect("domain WkbType always maps to a valid qgis WkbType");
            let epsg_id = match dto.epsg_id {
                domain::figure_layer::SupportedEpsg::BNG => SupportedEpsg::BNG,
                domain::figure_layer::SupportedEpsg::WGS84 => SupportedEpsg::WGS84,
            };
            QgisLayerSource::ProjectLayer(QgisProjectLayer::Valid {
                schema: dto.schema.clone(),
                table: dto.table.clone(),
                wkb_type,
                epsg_id,
            })
        }
        FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Invalid(dto)) => {
            QgisLayerSource::ProjectLayer(QgisProjectLayer::Invalid(dto.message.clone()))
        }
    };
    QgisLayerSpec {
        name: layer.name.clone(),
        styleqml: layer.styleqml.clone(),
        source,
        legend_text: layer.properties.legend_text.clone(),
        include_on_legend: layer.properties.include_on_legend,
        include_on_map: layer.properties.include_on_map,
        include_as_target: layer.properties.include_as_target,
        enable_labels: layer.properties.enable_labels,
        convert_boundary_to_singleparts: layer.properties.convert_boundary_to_singleparts,
    }
}

fn convert_properties(p: &domain::figure::FigureProperties) -> QgisFigureProperties {
    use domain::enums::{CopyrightText as DomainCopyright, ScalebarUnits as DomainScalebar};
    use qgis::enums::ScalebarUnits as QgisScalebar;

    QgisFigureProperties {
        title: p.title.clone(),
        subtitle: p.subtitle.clone(),
        extra_legend_text: p.extra_legend_text.clone(),
        enable_html: p.enable_html,
        figure_number: p.figure_number.clone(),
        scalebar_units: p.scalebar_units.as_ref().map(|u| match u {
            DomainScalebar::Kilometers => QgisScalebar::Kilometers,
            DomainScalebar::Meters => QgisScalebar::Meters,
        }),
        scalebar_units_per_segment: p.scalebar_units_per_segment,
        logo: p.logo,
        internal_use: p.internal_use,
        map_ticks: p.map_ticks,
        north_arrow: p.north_arrow,
        status: p.status.clone(),
        scalebar: p.scalebar,
        copyright_text: p.copyright_text.as_ref().map(|c| match c {
            DomainCopyright::Default => CopyrightText::Default,
            DomainCopyright::Custom => CopyrightText::Custom,
            DomainCopyright::None => CopyrightText::None,
        }),
        custom_copyright_text: p.custom_copyright_text.clone(),
        greyscale_background_map: p.greyscale_background_map,
        legend_text_vmargin: p.legend_text_vmargin,
        legend: p.legend,
        grid_spacing: p.grid_spacing,
        legend_height_percent: p.legend_height_percent,
        overview_frame: p.overview_frame,
        overview_map_scale: p.overview_map_scale,
    }
}

fn to_qgis_figure_config(cfg: &QgisFigureConfig) -> QgisConfig {
    QgisConfig {
        logo_path: format!("{}/{}", cfg.figure_assets_directory, cfg.logo_filename),
        logo_aspect_ratio: cfg.logo_height_pixels as f64 / cfg.logo_width_pixels as f64,
        north_arrow_path: format!(
            "{}/{}",
            cfg.figure_assets_directory, cfg.north_arrow_filename
        ),
    }
}

// ---------------------------------------------------------------------------
// QGIS server request helpers (ported from prototype)
// ---------------------------------------------------------------------------

/// Returns the filename to use in the Content-Disposition header.
fn figure_filename_with_id(figure: &FigureOutputDTO, suffix: &str) -> String {
    let mut name = String::from(figure.properties.title.as_deref().unwrap_or("untitled"));
    if let Some(n) = figure.properties.figure_number.as_deref() {
        name.push('_');
        name.push_str(n);
    }
    if let Some(s) = figure.properties.subtitle.as_deref() {
        name.push('_');
        name.push_str(s);
    }
    format!("{}_{:05}.{}", slug::slugify(name), figure.id.0, suffix)
}

/// Derive the QGIS project name from the figure UUID and resolution.
fn figure_qgis_project_name(figure: &FigureOutputDTO, resolution: &PrintResolution) -> String {
    match resolution {
        PrintResolution::High => figure.qgis_project_uuid.to_string(),
        PrintResolution::Low => format!("{}_low-res", figure.qgis_project_uuid),
    }
}

fn build_qgis_server_request(
    project_name: &str,
    figure: &FigureOutputDTO,
    qgis_settings: &QgisServerSettings,
    db_settings: &DatabaseSettings,
    client: &reqwest::Client,
    format: FigureFormat,
) -> RequestBuilder {
    let layout_name = {
        let mut name = String::from(figure.properties.title.as_deref().unwrap_or("untitled"));
        if let Some(subtitle) = &figure.properties.subtitle {
            name.push_str(&format!("-{}", subtitle));
        }
        slug::slugify(name)
    };

    let mut request = GetPrintRequestBuilder {
        project_name: project_name.to_string(),
        template: layout_name,
        db_name: db_settings.database_name.clone(),
    }
    .build();
    request.format = format;
    request.crs = "EPSG:27700".into();

    // Placeholder: map extent computed in build_figure_spec; here we use the
    // figure's computed extent directly via QgisFigureSpec methods in generate_project.
    // For the GetPrint WMS request we need layers and extent directly.
    let layers = figure
        .layers
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .filter(|l| l.properties.include_on_map)
        .filter(|l| {
            !matches!(
                l.source,
                FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Invalid(_))
            )
        })
        .map(|l| l.name.clone())
        .collect::<Vec<_>>();

    let target = target_coord(figure).unwrap_or(Point {
        x: 324636.0,
        y: 673221.0,
        srid: 27700,
    });
    let extent = map_extent(figure, target);
    let overview_extent = overview_map_extent(figure, target);

    // TODO: fetch base map slugs from DB when basemap IDs are set
    let base_map_slug: Option<String> = None;
    let overview_map_slug: Option<String> = None;

    let mut all_layers = layers;
    if let Some(ref slug) = base_map_slug {
        all_layers.push(slug.clone());
    }
    all_layers.reverse();

    let mut request_builder = client
        .get(qgis_settings.url.clone())
        .query(&request);

    let mut map_number = 0u32;

    if let Some(overview_slug) = overview_map_slug {
        if figure.legend_width_mm > 0 {
            request_builder = request_builder
                .query(&[(format!("map{}:layers", map_number), overview_slug)])
                .query(&[(
                    format!("map{}:extent", map_number),
                    overview_extent.to_string(),
                )]);
            map_number += 1;
        }
    }

    request_builder = request_builder.query(&[
        (format!("map{}:layers", map_number), all_layers.join(",")),
        (format!("map{}:extent", map_number), extent.to_string()),
    ]);

    if figure.properties.map_ticks.unwrap_or(false) {
        let grid_spacing = figure.properties.grid_spacing.unwrap_or(1000);
        request_builder = request_builder.query(&[
            (
                format!("map{}:GRID_INTERVAL_X", map_number),
                grid_spacing.to_string(),
            ),
            (
                format!("map{}:GRID_INTERVAL_Y", map_number),
                grid_spacing.to_string(),
            ),
        ]);
    }

    request_builder
}

async fn process_qgis_server_response(
    response: Response,
    filename: &str,
) -> Result<HttpResponse, ApiError> {
    if !response.status().is_success() {
        let status = response.status();
        let url = response.url().clone();
        match response.text().await {
            Ok(body) => {
                tracing::error!(
                    "QGIS server returned error {}: url={}, body={}",
                    status,
                    url,
                    body
                );
                return Err(ApiError::Unexpected(anyhow::anyhow!(
                    "QGIS server error ({}): {}",
                    status,
                    body
                )));
            }
            Err(e) => {
                tracing::error!(
                    "QGIS server returned error {} and body could not be read: {}",
                    status,
                    e
                );
                return Err(ApiError::Unexpected(anyhow::anyhow!(
                    "QGIS server error ({}): failed to read response body",
                    status
                )));
            }
        }
    }

    let content_disposition = format!("attachment; filename=\"{}\"", filename);
    let mut builder = streaming_response(&response);
    builder.insert_header(("Content-Disposition", content_disposition));
    Ok(builder.streaming(response.bytes_stream()))
}

fn streaming_response(response: &reqwest::Response) -> actix_web::HttpResponseBuilder {
    let status =
        actix_web::http::StatusCode::from_u16(response.status().as_u16()).unwrap_or_default();
    let mut builder = HttpResponse::build(status);
    for (name, value) in response.headers().iter().filter(|(h, _)| *h != "connection") {
        if let (Ok(name), Ok(value)) = (
            actix_web::http::header::HeaderName::from_bytes(name.as_ref()),
            actix_web::http::header::HeaderValue::from_bytes(value.as_ref()),
        ) {
            builder.insert_header((name, value));
        }
    }
    builder
}
