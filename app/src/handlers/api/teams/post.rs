use actix_web::{
    post,
    web::{self, Json},
};
use domain::{BusinessUnitId, TeamId};
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Serialize, Default)]
pub struct TeamInputPayload {
    pub name: String,
    pub business_unit: Option<BusinessUnitId>,
}

#[post("")]
#[tracing::instrument(skip(repo, user, payload))]
pub async fn post_team(
    repo: web::Data<PostgresRepo>,
    payload: web::Json<TeamInputPayload>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<Json<TeamId>, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let id = repo
        .insert(&(payload.into_inner(), user.into_inner().id))
        .await?;
    Ok(Json(id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MockUserCredentials, testing::test_helpers::mock_app};
    use actix_web::test;

    #[actix_web::test]
    async fn post_team_requires_admin_permission() {
        let req = test::TestRequest::post().set_json(&TeamInputPayload::default());
        let resp = mock_app(
            post_team,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
