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
};
use geo::{
    shapefile_processor::merge_geometries,
    virtual_shapefile::{ShapefileData, ShapefileError, ShapefileForm},
};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(MultipartForm)]
pub struct FeatureInputPayload {
    pub shp: TempFile,
    pub dbf: TempFile,
    pub shx: TempFile,
    pub prj: TempFile,
    pub name: actix_multipart::form::text::Text<String>,
    pub primary: Option<actix_multipart::form::text::Text<bool>>,
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
        name,
        primary,
    } = payload.into_inner();
    let shapefile = ShapefileForm { shp, dbf, shx, prj };
    let shapefile_data: ShapefileData = shapefile
        .try_into()
        .context("Unable to create shapefile data")
        .map_err(ShapefileError::UnexpectedError)?;
    let ds: Dataset = shapefile_data
        .try_into()
        .context("Unable to create GDAL dataset")
        .map_err(ShapefileError::UnexpectedError)?;
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
