use actix_web::{HttpResponse, delete, web};
use domain::BusinessUnitId;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[delete("/{bu_id}")]
#[tracing::instrument(skip(repo, user, id))]
pub async fn delete_business_unit(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<BusinessUnitId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.delete_business_unit(id.into_inner()).await?;
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
    async fn delete_business_unit_requires_admin_permission() {
        let req = test::TestRequest::delete().uri("/1");
        let resp = mock_app_with_path_params(
            delete_business_unit,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
