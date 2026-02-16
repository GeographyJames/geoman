use domain::{ProjectCollectionId, ProjectCollectionInputDto, UserId, enums::GeometryType};
use sqlx::{Acquire, Postgres};

use crate::repo::traits::Insert;

impl Insert for (&ProjectCollectionInputDto, UserId) {
    type Id = ProjectCollectionId;

    async fn insert<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let (collection, id) = self;
        let mut executor = conn.acquire().await?;
        sqlx::query_scalar!(
            r#"
            INSERT INTO app.collections (title, slug, description, geometry_type, added_by, last_updated_by)
            VALUES ($1, $2, $3, $4, $5, $5)
            RETURNING id AS "id: ProjectCollectionId"
            "#,
            collection.title.as_ref(),
            collection.slug,
            collection.description,
            &collection.geometry_type as &GeometryType,
            id.0
        ).fetch_one(&mut *executor).await.map_err(Into::into)
    }
}
