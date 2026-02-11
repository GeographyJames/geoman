use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{HttpResponse, post};
use anyhow::Context;
use domain::{FeatureInputDTO, UserId};
use gdal::{Dataset, vector::LayerAccess};
use geo::virtual_shapefile::{ShapefileData, ShapefileError, ShapefileForm};

use crate::{errors::ApiError, urls::Api};

#[derive(MultipartForm)]
pub struct FeatureInputPayload {
    pub shp: TempFile,
    pub dbf: TempFile,
    pub shx: TempFile,
    pub prj: TempFile,
    pub name: actix_multipart::form::text::Text<String>,
    pub primary: actix_multipart::form::text::Text<bool>,
}

impl TryFrom<FeatureInputPayload> for FeatureInputDTO {
    fn try_from(payload: FeatureInputPayload) -> Result<Self, Self::Error> {
        let shapefile_form = ShapefileForm {
            shp: payload.shp,
            dbf: payload.dbf,
            shx: payload.shx,
            prj: payload.prj,
        };

        let shapefile_data: ShapefileData = shapefile_form
            .try_into()
            .context("failed to create shapefile data")?;
        let ds: Dataset = shapefile_data.try_into()?;
        let mut layer = ds.layers().next().context("dataset has no layers")?;
        let first_feature = layer.features().next().context("layer has no features")?;
        let geom = first_feature
            .geometry()
            .context("feature has no geometry")?;
        todo!()
    }

    type Error = ShapefileError;
}

#[tracing::instrument()]
#[post("{projectId}/{collectionId}")]
pub async fn post_project_feature_shapefile() -> HttpResponse {
    HttpResponse::Ok().finish()
}
