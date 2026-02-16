use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{
    post,
    web::{self, Json},
};
use anyhow::Context;
use domain::{FeatureId, FeatureInputDTO, ProjectCollectionId, ProjectId};
use gdal::{
    Dataset,
    vector::{LayerAccess, OGRwkbGeometryType},
    vsi,
};
use geo::{
    shapefile_processor::merge_geometries,
    virtual_shapefile::{ShapefileData, ShapefileError, ShapefileForm},
};
use std::io::Read;
use uuid::Uuid;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(MultipartForm)]
pub struct FeatureInputPayload {
    pub shp: Option<TempFile>,
    pub dbf: Option<TempFile>,
    pub shx: Option<TempFile>,
    pub prj: Option<TempFile>,
    pub shz: Option<TempFile>,
    pub name: actix_multipart::form::text::Text<String>,
    pub primary: Option<actix_multipart::form::text::Text<bool>>,
}

fn dataset_from_shz(mut shz: TempFile) -> Result<Dataset, ShapefileError> {
    let mut bytes = Vec::new();
    shz.file
        .read_to_end(&mut bytes)
        .context("failed to read shz file")
        .map_err(ShapefileError::UnexpectedError)?;
    let path = format!("/vsimem/{}.shz", Uuid::new_v4());
    vsi::create_mem_file(&path, bytes)
        .context("failed to create virtual shz file")
        .map_err(ShapefileError::UnexpectedError)?;
    let ds = Dataset::open(&path)
        .context("failed to open shz dataset")
        .map_err(ShapefileError::UnexpectedError)?;
    let layer = ds.layers().next().context("no layers in shz file")
        .map_err(ShapefileError::UnexpectedError)?;
    // Force GDAL to read the spatial ref before unlinking
    let _srs = layer.spatial_ref();
    let _ = vsi::unlink_mem_file(&path);
    Ok(ds)
}

fn dataset_from_parts(
    shp: TempFile,
    dbf: TempFile,
    shx: TempFile,
    prj: TempFile,
) -> Result<Dataset, ShapefileError> {
    let shapefile = ShapefileForm { shp, dbf, shx, prj };
    let shapefile_data: ShapefileData = shapefile
        .try_into()
        .context("Unable to create shapefile data")
        .map_err(ShapefileError::UnexpectedError)?;
    let ds: Dataset = shapefile_data
        .try_into()
        .context("Unable to create GDAL dataset")
        .map_err(ShapefileError::UnexpectedError)?;
    Ok(ds)
}

#[tracing::instrument(skip(repo, payload))]
#[post("{projectId}/{collectionId}")]
pub async fn post_project_feature_shapefile(
    repo: web::Data<PostgresRepo>,
    payload: MultipartForm<FeatureInputPayload>,
    user: web::ReqData<AuthenticatedUser>,
    path: web::Path<(ProjectId, ProjectCollectionId)>,
) -> Result<Json<FeatureId>, ApiError> {
    let (project_id, collection_id) = path.into_inner();
    let projcet_srid = repo.get_project_srid(project_id).await?;
    let FeatureInputPayload {
        shp,
        dbf,
        shx,
        prj,
        shz,
        name,
        primary,
    } = payload.into_inner();
    let ds = match (shz, shp, dbf, shx, prj) {
        (Some(shz), None, None, None, None) => dataset_from_shz(shz)?,
        (None, Some(shp), Some(dbf), Some(shx), Some(prj)) => {
            dataset_from_parts(shp, dbf, shx, prj)?
        }
        _ => {
            return Err(ShapefileError::IncorrectFiles(
                "provide either a single .shz file or all four shapefile components (shp, dbf, shx, prj)".to_string(),
            ))?;
        }
    };
    let layer = ds.layers().next().unwrap();
    let srid = layer
        .spatial_ref()
        .context("no spatial reference")
        .map_err(ShapefileError::InvalidData)?
        .auth_code()
        .context("failed to retrive spatial ref auth code")
        .map_err(ShapefileError::InvalidData)?;
    let target_srid = projcet_srid.unwrap_or(srid);
    let geom_type = repo.get_collection_geom_type(collection_id).await?;
    let expected_type: OGRwkbGeometryType::Type = geom_type.into();
    let geom = merge_geometries(&ds, expected_type)?;

    let input_dto = FeatureInputDTO {
        name: name.0,
        primary: primary.map(|p| p.0),
        geom_wkb: geom
            .wkb()
            .context("failed to create WKB")
            .map_err(ShapefileError::UnexpectedError)?,
        srid,
        target_srid,
    };
    let feature_id = repo
        .insert(&(&input_dto, project_id, collection_id, user.id))
        .await?;

    Ok(Json(feature_id))
}
