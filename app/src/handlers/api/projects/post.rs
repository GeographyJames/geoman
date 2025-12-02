use actix_web::{
    HttpResponse, post,
    web::{self, Json},
};
use clerk_rs::validators::authorizer::ClerkJwt;
use domain::{ProjectId, project::ProjectInputDto};

use crate::{
    handlers::{ApiError, api::projects::ProjectReqPayload},
    postgres::PostgresRepo,
};

#[post("")]
#[tracing::instrument(skip(repo, payload))]
pub async fn post_projcet(
    repo: web::Data<PostgresRepo>,
    jwt: web::ReqData<ClerkJwt>,
    payload: Json<ProjectReqPayload>,
) -> Result<Json<ProjectId>, ApiError> {
    let input_dto = payload.into_inner().try_into_dto()?;
    let project_id = repo.insert(&(input_dto, jwt.into_inner())).await?;
    Ok(Json(project_id))
}
