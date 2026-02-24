use actix_web::{
    post,
    web::{self, Json},
};
use domain::BusinessUnitId;
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Serialize)]
pub struct BusinessUnitInputPayload {
    pub name: String,
}

#[post("")]
#[tracing::instrument(skip(repo, user, payload))]
pub async fn post_business_unit(
    repo: web::Data<PostgresRepo>,
    payload: web::Json<BusinessUnitInputPayload>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<Json<BusinessUnitId>, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let id = repo
        .insert(&(payload.into_inner(), user.into_inner().id))
        .await?;
    Ok(Json(id))
}
