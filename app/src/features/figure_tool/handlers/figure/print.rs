use actix_web::{HttpResponse, get, web};
use reqwest::RequestBuilder;

use crate::{
    config::{DatabaseSettings, QgisServerSettings},
    constants::QGIS_PROJECTS_SCHEMA,
    errors::ApiError,
    features::figure_tool::{
        FigureFormat, PrintResolution,
        dtos::{BaseMapOutputDTO, FigureOutputDTO, QgisProjectName},
        ids::FigureId,
        qgis_builder::generate_project,
    },
    postgres::PostgresRepo,
};
use qgis::layer::{PgConfig, SslMode};

#[derive(serde::Serialize)]
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

#[get("/{figure_id}/{format}")]
#[tracing::instrument(skip(repo, path, qgis_server, db_settings))]
pub async fn get_print(
    repo: web::Data<PostgresRepo>,
    path: web::Path<(FigureId, FigureFormat)>,
    qgis_server: web::Data<QgisServerSettings>,
    db_settings: web::Data<DatabaseSettings>,
    client: web::Data<reqwest::Client>,
) -> Result<HttpResponse, ApiError> {
    let (figure_id, format) = path.into_inner();
    let resolution = match format {
        FigureFormat::jpg => PrintResolution::Low,
        FigureFormat::pdf => PrintResolution::High,
    };
    let figure: FigureOutputDTO = repo
        .select_one::<FigureOutputDTO, _>(figure_id)
        .await?
        .ok_or(ApiError::NotFound)?;
    let filename = figure.filename_with_id(&format.clone().to_string());
    let project_name = figure.qgis_project_name(&resolution);

    // Check if a project already exists with this name
    let qgis_project = project_name.check_unique(&repo.db_pool).await?;
    // If the project does not exist, create it
    if qgis_project.is_none() {
        let qgis_project = generate_project(
            figure.clone(),
            Some(&qgis_server.figure_config),
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
        .map_err(|e| ApiError::Unexpected(e.context("failed to create qgis project")))?;
        repo.insert(&qgis_project).await?;
    }
    let request = build_request(project_name, figure, &repo, &qgis_server, &db_settings, &client, format).await?;
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
    qgis_server: &QgisServerSettings,
    db_settings: &DatabaseSettings,
    client: &reqwest::Client,
    format: FigureFormat,
) -> Result<RequestBuilder, ApiError> {
    let mut request = GetPrintRequestBuilder {
        project_name: project_name.0,
        pg_schema: QGIS_PROJECTS_SCHEMA.into(),
        template: figure.layout_name(),
        db_name: db_settings.database_name.clone(),
    }
    .build();
    request.format = format;
    request.crs = "EPSG:27700".into();

    let extent = figure.map_extent;
    let mut layers: Vec<String> = figure.map_layer_names();

    let mut conn = repo.db_pool.acquire().await.map_err(|e| {
        ApiError::Unexpected(anyhow::anyhow!("failed to acquire db connection: {}", e))
    })?;

    let base_map_slug = if let Some(BaseMapOutputDTO {
        id: base_map_id, ..
    }) = figure.main_map_base_map
    {
        let base_map = BaseMapOutputDTO::select(&mut conn, &base_map_id).await?;
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
        let overview_map = BaseMapOutputDTO::select(&mut conn, &overview_map_id).await?;
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

    let mut request_builder = client.get(qgis_server.url.clone()).query(&request);

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

async fn process_qgis_server_response(
    response: reqwest::Response,
    filename: &str,
) -> Result<HttpResponse, ApiError> {
    if !response.status().is_success() {
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
                )));
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
                )));
            }
        }
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_owned();

    let content_disposition = format!("attachment; filename=\"{}\"", filename);

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .insert_header(("Content-Disposition", content_disposition))
        .streaming(response.bytes_stream()))
}
