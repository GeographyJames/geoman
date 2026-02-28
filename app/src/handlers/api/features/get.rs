use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use domain::FeatureId;
use gdal::{
    spatial_ref::SpatialRef,
    vector::LayerAccess,
    vsi::{self, get_vsi_mem_file_bytes_owned},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FeatureFormat {
    #[default]
    Shapefile,
}

#[derive(Deserialize)]
pub struct FeatureDownloadPath {
    project_slug: String,
    collection_slug: String,
    feature_id: FeatureId,
}

#[derive(Deserialize)]
pub struct FeatureDownloadQuery {
    #[serde(default)]
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
    }
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

    let download_filename = format!(
        "{}-{}{:05}-{}.shz",
        project_slug,
        collection_slug,
        feature_id.0,
        slug::slugify(&ft.name)
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
