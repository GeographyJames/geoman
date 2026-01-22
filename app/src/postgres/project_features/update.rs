use domain::{ProjectFeatureId, ProjectId, UserId, enums::Status};

use crate::{
    handlers::ogc_api::features::patch::project_feature::PatchProjectFeaturePayload,
    repo::traits::Update,
};

impl Update
    for (
        &PatchProjectFeaturePayload,
        UserId,
        ProjectId,
        ProjectFeatureId,
    )
{
    type Id = ProjectFeatureId;

    async fn update<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let (payload, user_id, project_id, feature_id) = self;
        let result = sqlx::query!(
            r#"
        UPDATE app.project_features
        SET status = COALESCE($1, status),
            last_updated = NOW(),
            last_updated_by = $2
        WHERE id= $3
        AND collection_id = $4
        AND project_id = $5
        RETURNING id, collection_id"#,
            &payload.status as &Option<Status>,
            user_id.0,
            feature_id.id,
            feature_id.collection_id.0,
            project_id.0
        )
        .fetch_one(executor)
        .await?;
        Ok(ProjectFeatureId {
            collection_id: domain::ProjectCollectionId(result.collection_id),
            id: (result.id),
        })
    }
}
