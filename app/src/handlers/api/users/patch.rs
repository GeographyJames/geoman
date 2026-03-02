use actix_web::{
    HttpResponse, patch,
    web::{self},
};
use domain::{TeamId, UserId};
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Serialize, Deserialize, Default)]
pub struct PatchUserPayload {
    pub team_id: Option<TeamId>,
    pub admin: Option<bool>,
}

#[patch("/{user_id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_user(
    repo: web::Data<PostgresRepo>,
    body: web::Json<PatchUserPayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.update(&(body.into_inner(), id.into_inner())).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use crate::{
        AuthenticatedUser, MockUserCredentials, testing::test_helpers::mock_app_with_path_params,
    };

    use super::*;

    #[actix_web::test]
    async fn patch_user_requires_admin_permission() {
        let req = test::TestRequest::patch()
            .uri("/1")
            .set_json(&PatchUserPayload {
                team_id: None,
                admin: None,
            });
        let resp = mock_app_with_path_params(
            patch_user,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
