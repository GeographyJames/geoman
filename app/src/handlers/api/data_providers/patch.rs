use actix_web::{HttpResponse, patch, web};
use domain::DataProviderId;
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderUpdatePayload {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub country_code: Option<Option<String>>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub subdivision: Option<Option<String>>,
}

#[patch("/{id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_data_provider(
    repo: web::Data<PostgresRepo>,
    body: web::Json<DataProviderUpdatePayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<DataProviderId>,
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
    use super::*;
    use crate::{AuthenticatedUser, MockUserCredentials, testing::test_helpers::mock_app_with_path_params};
    use actix_web::test;

    #[actix_web::test]
    async fn patch_data_provider_requires_admin_permission() {
        let req = test::TestRequest::patch()
            .uri("/1")
            .set_json(&DataProviderUpdatePayload::default());
        let resp = mock_app_with_path_params(
            patch_data_provider,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
