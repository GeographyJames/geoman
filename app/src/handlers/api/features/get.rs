use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use domain::FeatureId;
use gdal::{
    spatial_ref::SpatialRef,
    vector::LayerAccess,
    vsi::{self, get_vsi_mem_file_bytes_owned},
};
use serde::Deserialize;
use strum::Display;
use uuid::Uuid;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum FeatureFormat {
    Shapefile,
    Csv,
}

impl FeatureFormat {
    fn file_extension(&self) -> String {
        match self {
            FeatureFormat::Shapefile => "shz".to_string(),
            FeatureFormat::Csv => "csv".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct FeatureDownloadPath {
    project_slug: String,
    collection_slug: String,
    feature_id: FeatureId,
}

#[derive(Deserialize)]
pub struct FeatureDownloadQuery {
    format: FeatureFormat,
}

#[tracing::instrument(skip(repo, path, query))]
#[get("/{project_slug}/{collection_slug}/{feature_id}")]
pub async fn get_project_feature_download(
    repo: web::Data<PostgresRepo>,
    path: web::Path<FeatureDownloadPath>,
    query: web::Query<FeatureDownloadQuery>,
    _user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    let FeatureDownloadPath {
        project_slug,
        collection_slug,
        feature_id,
    } = path.into_inner();
    match query.into_inner().format {
        FeatureFormat::Shapefile => {
            get_shapefile(&repo, feature_id, &project_slug, &collection_slug).await
        }
        FeatureFormat::Csv => get_csv(&repo, feature_id, &project_slug, &collection_slug).await,
    }
}

async fn get_csv(
    repo: &PostgresRepo,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<HttpResponse, ApiError> {
    let rows = repo
        .get_turbine_layout_csv(feature_id, project_slug, collection_slug)
        .await
        .context("failed to query turbine layout csv")?;

    if rows.is_empty() {
        return Err(ApiError::FeatureNotFound(feature_id));
    }

    let srid = rows[0].srid;
    let layout_name = &rows[0].layout_name;
    let mut csv = format!(
        "id,turbine_number,hub_height_m,rotor_diameter_m,x_epsg:{srid},y_epsg:{srid}\n"
    );
    let download_filename = download_filename(
        project_slug,
        collection_slug,
        feature_id,
        layout_name,
        FeatureFormat::Csv,
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            row.id,
            row.turbine_number,
            row.hub_height_m.map_or(String::new(), |v| v.to_string()),
            row.rotor_diameter_m
                .map_or(String::new(), |v| v.to_string()),
            row.x,
            row.y,
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", download_filename),
        ))
        .body(csv))
}

fn download_filename(
    project_slug: &str,
    collection_slug: &str,
    feature_id: FeatureId,
    feature_name: &str,
    format: FeatureFormat,
) -> String {
    format!(
        "{}-{}{:05}-{}.{}",
        project_slug,
        collection_slug,
        feature_id.0,
        slug::slugify(feature_name),
        format.file_extension()
    )
}
async fn get_shapefile(
    repo: &PostgresRepo,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<HttpResponse, ApiError> {
    let ft = repo
        .get_project_feature_for_download(feature_id, project_slug, collection_slug)
        .await
        .context("failed to query project feature")?
        .ok_or(ApiError::FeatureNotFound(feature_id))?;

    let download_filename = download_filename(
        project_slug,
        collection_slug,
        feature_id,
        &ft.name,
        FeatureFormat::Shapefile,
    );

    let vsimem_path = format!("/vsimem/{}.shz", Uuid::new_v4());

    let driver = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
        .context("failed to get gdal shapefile driver")?;
    let mut dataset = driver
        .create_vector_only(&vsimem_path)
        .context("failed to create vector layer")?;
    let srs = SpatialRef::from_epsg(ft.srid as u32).context("failed to get spatial ref")?;
    let layer_name = format!(
        "{}{:05}-{}",
        collection_slug,
        feature_id.0,
        slug::slugify(ft.name)
    );
    let layer_options = gdal::vector::LayerOptions {
        name: &layer_name,
        srs: Some(&srs),
        ty: ft.geom_type.into(),
        options: None,
    };
    let mut layer = dataset
        .create_layer(layer_options)
        .context("failed to create layer")?;
    let geom = gdal::vector::Geometry::from_wkb(&ft.geom)
        .context("failed to generate gdal geom from wkb")?;
    layer
        .create_feature(geom)
        .context("failed to add feature to layer")?;
    dataset.flush_cache().context("failed to flush cache")?;
    dataset.close().context("failed to close dataset")?;

    let content =
        get_vsi_mem_file_bytes_owned(&vsimem_path).context("failed to get bytes owned")?;
    let _ = vsi::unlink_mem_file(&vsimem_path);

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", download_filename),
        ))
        .body(content))
}
