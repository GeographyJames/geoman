use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{
    post,
    web::{self, Json},
};
use anyhow::Context;
use domain::{
    FeatureInputDTO, ProjectCollectionId, ProjectId, builder::InputDTOBuilder, name::NameInputDTO,
    turbine_layout::TurbineLayoutInputDTO,
};
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

use crate::{
    AuthenticatedUser, constants::TURBINE_LAYOUTS_COLLECTION_ID, errors::ApiError,
    handlers::api::features::payload::FeatureInputPayload, postgres::PostgresRepo,
};

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
    let layer = ds
        .layers()
        .next()
        .context("no layers in shz file")
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

#[tracing::instrument(skip(repo, payload, user, path))]
#[post("{projectId}/{collectionId}")]
pub async fn post_project_feature_shapefile(
    repo: web::Data<PostgresRepo>,
    payload: MultipartForm<FeatureInputPayload>,
    user: web::ReqData<AuthenticatedUser>,
    path: web::Path<(ProjectId, ProjectCollectionId)>,
) -> Result<Json<i32>, ApiError> {
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
        hub_height_default_metre,
        rotor_diameter_default_metre,
        turbine_number_field,
        rotor_diameter_field,
        hub_height_field,
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
    let primary = primary.map(|p| p.0);
    let name = NameInputDTO::parse(name.into_inner()).map_err(|e| ApiError::InvalidName(e))?;
    if collection_id.0 == TURBINE_LAYOUTS_COLLECTION_ID {
        let hub_height_default_mm =
            hub_height_default_metre.map(|v| (v.into_inner() * 1000.) as u32);
        let blade_length_default_mm =
            rotor_diameter_default_metre.map(|v| (v.into_inner() * 1000.) as u32);
        let turbine_number_field = turbine_number_field.map(|v| v.into_inner());
        let blade_length_field = rotor_diameter_field.map(|v| v.into_inner());
        let hub_height_field = hub_height_field.map(|v| v.into_inner());
        let builder = InputDTOBuilder::new(&ds)?;
        let turbines = builder.build_turbines_geom_input_dto(
            hub_height_default_mm,
            blade_length_default_mm,
            turbine_number_field,
            blade_length_field,
            hub_height_field,
        )?;
        let input_dto = TurbineLayoutInputDTO {
            name,
            primary,
            turbines,
            target_srid,
            srid,
        };
        let layout_id = repo.insert(&(&input_dto, project_id, user.id)).await?;
        Ok(Json(layout_id.0))
    } else {
        let geom_type = repo.get_collection_geom_type(collection_id).await?;
        let expected_type: OGRwkbGeometryType::Type = geom_type.into();
        let geom = merge_geometries(&ds, expected_type)?;
        let input_dto = FeatureInputDTO {
            name,
            primary,
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
        Ok(Json(feature_id.0))
    }
}
