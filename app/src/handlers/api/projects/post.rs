use actix_web::{
    post,
    web::{self, Json},
};

use domain::{ProjectId, project::ProjectInputDto};

use crate::{
    handlers::{ApiError, api::projects::ProjectReqPayload},
    postgres::PostgresRepo,
    types::AuthenticatedUser,
};

#[post("")]
#[tracing::instrument(skip(repo, payload, user))]
pub async fn post_project(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    payload: Json<ProjectReqPayload>,
) -> Result<Json<ProjectId>, ApiError> {
    let AuthenticatedUser { id, team_id, .. } = user.into_inner();
    let team_id = team_id.ok_or_else(|| todo!()).unwrap();
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
        let mut user = AuthenticatedUser::default();
        user.team_id = Some(TeamId(0));
        let resp = mock_app(post_project, req, user).await;
        assert_eq!(resp.status(), 422);
        let error: ErrorResponse = test::read_body_json(resp).await;
        assert!(error.message.contains("Failed to validate project"))
    }

    #[actix_web::test]
    async fn post_project_return_422_for_integer_name() {
        let mut project = ProjectReqPayload::default();
        project.name = 1234.to_string();
        let mut user = AuthenticatedUser::default();
        user.team_id = Some(TeamId(0));
        let req = test::TestRequest::post().set_json(project);
        let resp = mock_app(post_project, req, user).await;
        assert_eq!(resp.status(), 422);
    }
    #[tokio::test]
    #[ignore]
    async fn post_project_returns_403_for_user_without_team() {
        let project = ProjectReqPayload::default();
        let req = test::TestRequest::post().set_json(project);
        let resp = mock_app(post_project, req, AuthenticatedUser::default()).await;
        assert_eq!(resp.status(), 403)
    }
}
