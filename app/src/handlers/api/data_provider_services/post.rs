use actix_web::{
    post,
    web::{self, Json},
};
use domain::enums::DataProviderServiceType;
use domain::{DataProviderId, DataProviderServiceId};
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderServiceInputPayload {
    pub provider_id: DataProviderId,
    pub name: String,
    pub service_type: DataProviderServiceType,
    pub base_url: String,
}

#[post("")]
#[tracing::instrument(skip(repo, user, payload))]
pub async fn post_data_provider_service(
    repo: web::Data<PostgresRepo>,
    payload: web::Json<DataProviderServiceInputPayload>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<Json<DataProviderServiceId>, ApiError> {
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
    async fn post_data_provider_service_requires_admin_permission() {
        let req = test::TestRequest::post().set_json(&DataProviderServiceInputPayload::default());
        let resp = mock_app(
            post_data_provider_service,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
