use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use domain::{FeatureId, enums::GeometryType};
use gdal::{
    spatial_ref::SpatialRef,
    vector::LayerAccess,
    vsi::{self, get_vsi_mem_file_bytes_owned},
};
use uuid::Uuid;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

pub struct ProjectFeature {
    name: String,
    geom: Vec<u8>,
    srid: i32,
    geom_type: GeometryType,
    project_slug: String,
    collection_slug: String,
}

#[tracing::instrument(skip(repo))]
#[get("/{featureId}")]
pub async fn get_project_feature_shapefile(
    repo: web::Data<PostgresRepo>,
    path: web::Path<FeatureId>,
    _user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    let feature_id = path.into_inner();
    let ft = sqlx::query_as!(
        ProjectFeature,
        r#"
    SELECT
            pf.name,
            ST_AsBinary(geom) AS "geom!",
            ST_Srid(geom) AS "srid!",
            GeometryType(geom) AS "geom_type!: GeometryType",
            p.slug AS project_slug,
            c.slug AS collection_slug
    FROM app.project_features pf
    JOIN app.projects p ON p.id = pf.project_id
    JOIN app.collections c ON c.id = pf.collection_id
    WHERE pf.id = $1
    "#,
        feature_id.0
    )
    .fetch_optional(&repo.db_pool)
    .await
    .context("failed to query project feature")?
    .ok_or(ApiError::FeatureNotFound(feature_id))?;

    let download_filename = format!(
        "{}-{}{:05}-{}.shz",
        ft.project_slug,
        ft.collection_slug,
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
        ft.collection_slug,
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
