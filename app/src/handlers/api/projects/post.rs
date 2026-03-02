use actix_web::{
    post,
    web::{self, Json},
};

use domain::{ProjectId, project::ProjectInputDto};

use crate::{
    constants::UNASSIGNED_USERS_TEAM_ID,
    handlers::{ApiError, api::projects::PostProjectPayload},
    postgres::PostgresRepo,
    types::AuthenticatedUser,
};

#[post("")]
#[tracing::instrument(skip(repo, payload, user))]
pub async fn post_project(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    payload: Json<PostProjectPayload>,
) -> Result<Json<ProjectId>, ApiError> {
    if user.team_id.0 == UNASSIGNED_USERS_TEAM_ID {
        return Err(ApiError::UnassignedUser);
    }
    let input_dto: ProjectInputDto = payload.into_inner().try_into()?;
    let project_id = repo.insert(&(&input_dto, user.id)).await?;
    Ok(Json(project_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ErrorResponse, MockUserCredentials, constants::UNASSIGNED_USERS_TEAM_ID,
        testing::test_helpers::mock_app,
    };
    use actix_web::test;

    #[actix_web::test]
    async fn post_project_returns_422_for_invalid_name() {
        let mut project = PostProjectPayload::default();
        project.name = "".to_string();
        let req = test::TestRequest::post().set_json(project).uri("/");

        let resp = mock_app(
            post_project,
            req,
            MockUserCredentials::Token("user_test123".to_string()),
        )
        .await;
        assert_eq!(resp.status(), 422);
        let error: ErrorResponse = test::read_body_json(resp).await;
        assert!(error.message.contains("Invalid project name"))
    }

    #[actix_web::test]
    async fn post_project_return_422_for_integer_name() {
        let mut project = PostProjectPayload::default();
        project.name = 1234.to_string();

        let req = test::TestRequest::post().set_json(project);
        let resp = mock_app(
            post_project,
            req,
            MockUserCredentials::Token("user_test123".to_string()),
        )
        .await;
        assert_eq!(resp.status(), 422);
    }

    #[actix_web::test]
    async fn post_project_returns_401_for_unassigned_user() {
        let mut user = AuthenticatedUser::default();
        user.team_id = domain::TeamId(UNASSIGNED_USERS_TEAM_ID);
        let req = test::TestRequest::post().set_json(PostProjectPayload::default());
        let resp = mock_app(post_project, req, MockUserCredentials::User(user)).await;
        assert_eq!(resp.status(), 401);
    }
}
