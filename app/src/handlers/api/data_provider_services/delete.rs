use actix_web::{HttpResponse, delete, web};
use domain::DataProviderServiceId;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[delete("/{id}")]
#[tracing::instrument(skip(repo, user, id))]
pub async fn delete_data_provider_service(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<DataProviderServiceId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.delete_data_provider_service(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthenticatedUser, MockUserCredentials, testing::test_helpers::mock_app_with_path_params};
    use actix_web::test;

    #[actix_web::test]
    async fn delete_data_provider_service_requires_admin_permission() {
        let req = test::TestRequest::delete().uri("/1");
        let resp = mock_app_with_path_params(
            delete_data_provider_service,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
