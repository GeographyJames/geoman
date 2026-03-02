use actix_web::{HttpResponse, patch, web};
use domain::BusinessUnitId;
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Serialize, Deserialize)]
pub struct BusinessUnitUpdatePayload {
    pub name: Option<String>,
}

#[patch("/{bu_id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_business_unit(
    repo: web::Data<PostgresRepo>,
    body: web::Json<BusinessUnitUpdatePayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<BusinessUnitId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let user = user.into_inner();
    repo.update(&(body.into_inner(), id.into_inner(), user.id))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use crate::{AuthenticatedUser, MockUserCredentials, testing::test_helpers::mock_app_with_path_params};

    use super::*;

    #[actix_web::test]
    async fn patch_business_unit_requires_admin_permission() {
        let req = test::TestRequest::patch()
            .uri("/1")
            .set_json(&BusinessUnitUpdatePayload { name: None });
        let resp = mock_app_with_path_params(
            patch_business_unit,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
