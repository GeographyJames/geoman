use actix_web::{
    post,
    web::{self, Json},
};
use domain::{ProjectId, UserId};

use crate::{
    handlers::{ApiError, api::projects::ProjectReqPayload},
    postgres::PostgresRepo,
};

#[post("")]
#[tracing::instrument(skip(repo, payload, user_id))]
pub async fn post_projcet(
    repo: web::Data<PostgresRepo>,
    user_id: web::ReqData<UserId>,
    payload: Json<ProjectReqPayload>,
) -> Result<Json<ProjectId>, ApiError> {
    let input_dto = payload.into_inner().try_into_dto()?;
    let project_id = repo.insert(&(input_dto, user_id.into_inner())).await?;
    Ok(Json(project_id))
}
