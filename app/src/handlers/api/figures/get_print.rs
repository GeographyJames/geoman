use std::fmt::Display;

use actix_web::{HttpResponse, web};
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};

use crate::{
    app::{
        configuration::Settings,
        handlers::api::{ApiError, helpers::streaming_response},
    },
    domain::dtos::{BaseMapOutputDTO, FigureOutputDTO, Id, QgisProjectName},
    postgres::PostgresRepo,
    qgis::{
        figure::{PrintResolution, generate_project},
        layer::{PgConfig, SslMode},
    },
};

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum FigureFormat {
    pdf,
    jpg,
}

impl Display for FigureFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureFormat::jpg => write!(f, "jpg"),
            FigureFormat::pdf => write!(f, "pdf"),
        }
    }
}

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
            map: "postgresql://?dbname=geodata_local&schema=qgis&project=test-project".into(),
        }
    }
}

pub struct GetPrintRequestBuilder {
    pub project_name: String,
    pub pg_schema: String,
    pub template: String,
    pub db_name: String,
}

impl GetPrintRequestBuilder {
    pub fn build(self) -> GetPrintRequest {
        GetPrintRequest {
            map: format!(
                "postgresql://?dbname={}&schema={}&project={}",
                self.db_name, self.pg_schema, self.project_name
            ),
            template: self.template,
            ..Default::default()
        }
    }
}

#[tracing::instrument(skip(repo, path, config))]
pub async fn get_print(
    repo: web::Data<PostgresRepo>,
    path: web::Path<(Id, FigureFormat)>,
    config: web::Data<Settings>,
    client: web::Data<reqwest::Client>,
) -> Result<HttpResponse, actix_web::Error> {
    let (figure_id, format) = path.into_inner();
    let resolution = match format {
        FigureFormat::jpg => PrintResolution::Low,
        FigureFormat::pdf => PrintResolution::High,
    };
    let figure: FigureOutputDTO =
        repo.select(&figure_id)
            .await
            .map_err(|e| ApiError::Repository {
                source: e,
                message: "failed to retrieve figure from database".into(),
            })?;
    let filename = figure.filename_with_id(&format.clone().to_string());
    let project_name = figure.qgis_project_name(&resolution);

    // Check if a project already exists with this name
    let qgis_project = check_for_project(&repo, &project_name).await?;
    // If the project does not exist, create it
    if qgis_project.is_none() {
        let qgis_project = generate_project(
            figure.clone(),
            Some(&config.qgis_server.figure_config),
            &resolution,
            true,
            PgConfig {
                db_name: config.database.database_name.clone(),
                port: config.database.port,
                host: config.database.host.clone(),
                sslmode: SslMode::from(config.database.require_ssl),
            },
            None,
        )
        .map_err(|e| ApiError::Unexpected(e.context("failed to create qgis project")))?;
        repo.insert(&qgis_project)
            .await
            .map_err(|e| ApiError::Repository {
                source: e,
                message: "failed to add qgis project to database".into(),
            })?;
    }
    let request = build_request(project_name, figure, &repo, &config, &client, format).await?;
    let response = request.send().await.map_err(|e| {
        ApiError::Unexpected(anyhow::anyhow!(
            "failed to execute request for PDF to QGIS server: {:?}",
            e
        ))
    })?;

    process_qgis_server_response(response, &filename).await
}

async fn build_request(
    project_name: QgisProjectName,
    figure: FigureOutputDTO,
    repo: &PostgresRepo,
    config: &Settings,
    client: &reqwest::Client,
    format: FigureFormat,
) -> Result<RequestBuilder, ApiError> {
    let mut request = GetPrintRequestBuilder {
        project_name: project_name.0,
        pg_schema: "qgis".into(),
        template: figure.layout_name(),
        db_name: config.database.database_name.clone(),
    }
    .build();
    request.format = format;
    request.crs = "EPSG:27700".into();

    let extent = figure.map_extent;

    let mut layers = figure.map_layer_names();

    let base_map_slug = if let Some(BaseMapOutputDTO {
        id: base_map_id, ..
    }) = figure.main_map_base_map
    {
        let base_map: BaseMapOutputDTO =
            repo.select(&base_map_id)
                .await
                .map_err(|e| ApiError::Repository {
                    source: e,
                    message: "failed to retrieve base map from database".into(),
                })?;
        if base_map.datasource.is_some() {
            Some(base_map.slug)
        } else {
            None
        }
    } else {
        None
    };
    let overview_map_slug = if let Some(BaseMapOutputDTO {
        id: overview_map_id,
        ..
    }) = figure.overview_map_base_map
    {
        let overview_map: BaseMapOutputDTO =
            repo.select(&overview_map_id)
                .await
                .map_err(|e| ApiError::Repository {
                    source: e,
                    message: "failed to retrieve overview map from database".into(),
                })?;
        if overview_map.datasource.is_some() {
            Some(overview_map.overview_map_slug())
        } else {
            None
        }
    } else {
        None
    };
    if let Some(ref slug) = base_map_slug {
        layers.push(slug.clone());
    }

    layers.reverse();

    let mut request_builder = client.get(config.qgis_server.url.clone()).query(&request);

    let mut map_number = 0;

    if let Some(overview_slug) = overview_map_slug
        && figure.legend_width_mm > 0
    {
        let overview_map_extent = figure.overview_map_extent;
        request_builder =
            request_builder.query(&[(format!("map{}:layers", map_number), overview_slug)]);
        request_builder = request_builder.query(&[(
            format!("map{}:extent", map_number),
            overview_map_extent.to_string(),
        )]);
        map_number += 1;
    }

    request_builder = request_builder.query(&[
        (format!("map{}:layers", map_number), layers.join(",")),
        (format!("map{}:extent", map_number), extent.to_string()),
    ]);

    if figure.properties.map_ticks.unwrap_or(false) {
        let grid_spacing = figure.properties.grid_spacing.unwrap_or(1000);
        request_builder = request_builder.query(&[
            (format!("map{}:GRID_INTERVAL_X", map_number), grid_spacing),
            (format!("map{}:GRID_INTERVAL_Y", map_number), grid_spacing),
        ])
    }

    Ok(request_builder)
}

async fn check_for_project(
    repo: &PostgresRepo,
    project_name: &QgisProjectName,
) -> Result<Option<QgisProjectName>, ApiError> {
    repo.check_unique(project_name)
        .await
        .map_err(|e| ApiError::Repository {
            source: e,
            message: "failed to check database for qgis project".into(),
        })
}

async fn process_qgis_server_response(
    response: Response,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    if !response.status().is_success() {
        // Try to read the error response body for logging
        let status = response.status();
        let url = response.url().clone();

        match response.text().await {
            Ok(error_body) => {
                tracing::error!(
                    "QGIS server returned error status {}: URL: {}, Response: {}",
                    status,
                    url,
                    error_body
                );
                return Err(ApiError::Unexpected(anyhow::anyhow!(
                    "QGIS server error ({}): {}",
                    status,
                    error_body
                ))
                .into());
            }
            Err(e) => {
                tracing::error!(
                    "QGIS server returned error status {} and failed to read response body: {}",
                    status,
                    e
                );
                return Err(ApiError::Unexpected(anyhow::anyhow!(
                    "QGIS server error ({}): Failed to read error response",
                    status
                ))
                .into());
            }
        }
    }

    // Set filename based on format
    let content_disposition = format!("attachment; filename=\"{}\"", filename);

    let mut response_builder = streaming_response(&response);
    response_builder.insert_header(("Content-Disposition", content_disposition));
    Ok(response_builder.streaming(response.bytes_stream()))
}
