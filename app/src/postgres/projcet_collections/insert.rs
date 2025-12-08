use domain::{ProjectCollectionId, ProjectCollectionInputDto, UserId, enums::GeometryType};

use crate::repo::traits::Insert;

impl Insert for (&ProjectCollectionInputDto, UserId) {
    type Id = ProjectCollectionId;

    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        let (collection, id) = self;
        sqlx::query_scalar!(
            r#"
            INSERT INTO app.collections (title, description, geometry_type, added_by, last_updated_by)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id AS "id: ProjectCollectionId"
            "#,
           
            collection.title,
            collection.description,
            &collection.geometry_type as &GeometryType,
            id.0
        ).fetch_one(executor).await.map_err(Into::into)
    }
}
