use actix_web::{HttpResponse, delete, web};
use domain::UserId;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[delete("/{user_id}")]
#[tracing::instrument(skip(repo, user, id))]
pub async fn delete_user(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let target_id = id.into_inner();
    if target_id == user.id {
        return Err(ApiError::AdminOnly);
    }
    repo.delete_user(target_id).await?;
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
    async fn delete_user_requires_admin_permission() {
        let req = test::TestRequest::delete().uri("/1");
        let resp = mock_app_with_path_params(
            delete_user,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn admin_cannot_delete_themselves() {
        let mut admin = AuthenticatedUser::default();
        admin.admin = true;
        admin.id = UserId(1);
        let req = test::TestRequest::delete().uri("/1");
        let resp =
            mock_app_with_path_params(delete_user, req, MockUserCredentials::User(admin)).await;
        assert_eq!(resp.status(), 401);
    }
}
