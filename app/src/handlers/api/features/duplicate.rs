use actix_web::{HttpResponse, post, web};
use domain::{
    FeatureId, FeatureNameInputDTO, LayoutId, ProjectCollectionId, ProjectId,
    turbine_layout::DuplicateTurbineInputDTO,
};
use serde::{Deserialize, Serialize};

use crate::{
    AuthenticatedUser, constants::TURBINE_LAYOUTS_COLLECTION_ID, errors::ApiError,
    postgres::PostgresRepo,
};

#[derive(Deserialize, Serialize, Default)]
pub struct DuplicateLayoutBody {
    pub name: Option<String>,
    pub hub_height_metre: Option<f64>,
    pub rotor_diameter_metre: Option<f64>,
    pub primary: Option<bool>,
}

#[post("{projectId}/{collectionId}/{featureId}/duplicate")]
#[tracing::instrument(skip(repo, path, user, body))]
pub async fn duplicate_project_feature(
    path: web::Path<(ProjectId, ProjectCollectionId, FeatureId)>,
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    body: web::Json<DuplicateLayoutBody>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, collection_id, feature_id) = path.into_inner();
    let payload = body.into_inner();

    if collection_id.0 == TURBINE_LAYOUTS_COLLECTION_ID {
        let name = payload
            .name
            .map(|n| FeatureNameInputDTO::parse(n).map_err(ApiError::InvalidName))
            .transpose()?;
        let hub_height_mm = payload.hub_height_metre.map(|v| (v * 1000.) as i32);
        let rotor_diameter_mm = payload.rotor_diameter_metre.map(|v| (v * 1000.) as i32);

        let dto = DuplicateTurbineInputDTO {
            name,
            hub_height_mm,
            rotor_diameter_mm,
            primary: payload.primary,
        };
        let new_id = repo
            .insert(&(&dto, project_id, LayoutId(feature_id.0), user.id))
            .await?;

        return Ok(HttpResponse::Created().json(new_id.0));
    }

    Err(ApiError::CollectionNotFound)
}
