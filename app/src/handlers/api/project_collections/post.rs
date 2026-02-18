use actix_web::{
    post,
    web::{self, Json},
};

use domain::{ProjectCollectionId, ProjectCollectionInputDto};

use crate::{
    errors::ApiError, handlers::api::project_collections::CollectionReqPayload,
    postgres::PostgresRepo, types::AuthenticatedUser,
};

#[post("")]
#[tracing::instrument(skip(repo, payload, user))]
pub async fn post_project_collection(
    repo: web::Data<PostgresRepo>,
    payload: Json<CollectionReqPayload>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<Json<ProjectCollectionId>, ApiError> {
    let collection_input_dto: ProjectCollectionInputDto = payload
        .into_inner()
        .try_into()
        .map_err(ApiError::InvalidCollectionTitle)?;
    let collection_id = repo.insert(&(&collection_input_dto, user.id)).await?;
    Ok(Json(collection_id))
}
