use actix_web::{HttpResponse, patch, web};
use domain::DataProviderServiceId;
use domain::enums::DataProviderServiceType;
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderServiceUpdatePayload {
    pub name: Option<String>,
    pub service_type: Option<DataProviderServiceType>,
    pub base_url: Option<String>,
}

#[patch("/{id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_data_provider_service(
    repo: web::Data<PostgresRepo>,
    body: web::Json<DataProviderServiceUpdatePayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<DataProviderServiceId>,
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
    use crate::{
        AuthenticatedUser, MockUserCredentials, testing::test_helpers::mock_app_with_path_params,
    };
    use actix_web::test;

    #[actix_web::test]
    async fn patch_data_provider_service_requires_admin_permission() {
        let req = test::TestRequest::patch()
            .uri("/1")
            .set_json(&DataProviderServiceUpdatePayload::default());
        let resp = mock_app_with_path_params(
            patch_data_provider_service,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
