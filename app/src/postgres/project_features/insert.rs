use domain::{FeatureId, FeatureInputDTO, ProjectCollectionId, ProjectId, UserId};

use crate::repo::traits::Insert;

impl Insert for (&FeatureInputDTO, ProjectId, ProjectCollectionId, UserId) {
    type Id = FeatureId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let (dto, project_id, collection_id, user_id) = self;
        let feature_id: FeatureId = sqlx::query_scalar!(
            r#"

                INSERT INTO app.project_features (
                            project_id,
                            collection_id,
                            name,
                            added_by,
                            last_updated_by,
                            is_primary,
                            geom
                            )
                            VALUES ($1, $2, $3, $4, $4, COALESCE($5, false), ST_GeomFromWKB($6, $7)) RETURNING id AS "id: FeatureId"

        "#,
            project_id.0,
            collection_id.0,
            dto.name,
            user_id.0,
            dto.primary,
            dto.geom_wkb,
            dto.srid
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(feature_id)
    }
}
