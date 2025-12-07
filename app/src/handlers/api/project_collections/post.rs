use actix_web::{
    post,
    web::{self, Json},
};

use domain::{ProjectCollectionId, ProjectCollectionInputDto};

use crate::{
    errors::ApiError,
    handlers::api::project_collections::CollectionReqPayload,
    helpers::get_user_context,
    postgres::PostgresRepo,
    types::{AuthenticatedUser, UserClient},
};

#[post("")]
#[tracing::instrument(skip(repo, payload, user, user_client))]
pub async fn post_project_collection(
    repo: web::Data<PostgresRepo>,
    payload: Json<CollectionReqPayload>,
    user: web::ReqData<AuthenticatedUser>,
    user_client: web::Data<UserClient>,
) -> Result<Json<ProjectCollectionId>, ApiError> {
    let collection_input_dto: ProjectCollectionInputDto = payload.into_inner().into();
    let user_context = get_user_context(&repo, user.into_inner(), &user_client).await?;
    let collection_id = repo
        .insert(&(collection_input_dto, user_context.id))
        .await?;
    Ok(Json(collection_id))
}
