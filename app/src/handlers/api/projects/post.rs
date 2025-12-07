use actix_web::{
    post,
    web::{self, Json},
};

use domain::{ProjectId, enums::Action, project::ProjectInputDto};

use crate::{
    handlers::{ApiError, api::projects::ProjectReqPayload},
    helpers::get_user_context,
    postgres::PostgresRepo,
    types::{AuthenticatedUser, UserClient, UserContext},
};

#[post("")]
#[tracing::instrument(skip(repo, payload, user, user_client))]
pub async fn post_project(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    payload: Json<ProjectReqPayload>,
    user_client: web::Data<UserClient>,
) -> Result<Json<ProjectId>, ApiError> {
    let user_context = get_user_context(&repo, user.into_inner(), &user_client).await?;
    let UserContext { id, team_id, .. } = user_context;
    let team_id = team_id.ok_or_else(|| ApiError::UserWithoutTeam(Action::CreateProject))?;
    let input_dto: ProjectInputDto = payload.into_inner().try_into()?;
    let project_id = repo.insert(&(&input_dto, id, team_id)).await?;
    Ok(Json(project_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ErrorResponse, testing::test_helpers::mock_app};
    use actix_web::test;
    use domain::TeamId;

    #[actix_web::test]
    async fn post_project_returns_422_for_invalid_name() {
        let mut project = ProjectReqPayload::default();
        project.name = "".to_string();
        let req = test::TestRequest::post().set_json(project);
        let mut user_context = UserContext::default();
        user_context.team_id = Some(TeamId(0));
        let resp = mock_app(post_project, req, AuthenticatedUser::User(user_context)).await;
        assert_eq!(resp.status(), 422);
        let error: ErrorResponse = test::read_body_json(resp).await;
        assert!(error.message.contains("Failed to validate project"))
    }

    #[actix_web::test]
    async fn post_project_return_422_for_integer_name() {
        let mut project = ProjectReqPayload::default();
        project.name = 1234.to_string();
        let mut user_context = UserContext::default();
        user_context.team_id = Some(TeamId(0));
        let req = test::TestRequest::post().set_json(project);
        let resp = mock_app(post_project, req, AuthenticatedUser::User(user_context)).await;
        assert_eq!(resp.status(), 422);
    }

    #[tokio::test]
    async fn post_project_returns_403_for_user_without_team() {
        let project = ProjectReqPayload::default();
        let req = test::TestRequest::post().set_json(project);
        let user_context = UserContext::default();
        let resp = mock_app(post_project, req, AuthenticatedUser::User(user_context)).await;
        assert_eq!(resp.status(), 403)
    }
}
