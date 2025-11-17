use domain::{Collection, ProjectId};

use crate::{
    errors::RepositoryError,
    postgres::traits::{SelectAll, SelectAllWithParams, SelectOne, SelectOneWithParams},
};

impl SelectAll for Collection {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            Collection,
            "SELECT id, title, slug, description FROM app.collections ORDER BY id"
        )
        .fetch_all(executor)
        .await
        .map_err(RepositoryError::from)
    }
}

impl SelectOne for Collection {
    type Id<'a> = &'a str;
    async fn select_one<'a, 'e, E>(
        executor: E,
        slug: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            Collection,
            "SELECT id, title, slug, description FROM app.collections WHERE slug = $1",
            slug
        )
        .fetch_optional(executor)
        .await
        .map_err(RepositoryError::from)
    }
}

pub struct SelectOneParams {
    pub project_id: ProjectId,
}

impl SelectOneWithParams for Collection {
    type Params = SelectOneParams;
    type Id<'a> = &'a str;
    async fn select_one_with_params<'a, 'e, E>(
        executor: E,
        slug: Self::Id<'a>,
        params: Self::Params,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            Collection,
            "  SELECT id, title, slug, description
  FROM app.collections c
  WHERE EXISTS (
      SELECT 1
      FROM app.project_features f
      WHERE f.collection_id = c.id 
      AND f.project_id =  $1
  )
  AND slug = $2",
            params.project_id.0,
            slug
        )
        .fetch_optional(executor)
        .await
        .map_err(RepositoryError::from)
    }
}

pub struct SelectAllParams {
    pub project_id: ProjectId,
}

impl SelectAllWithParams for Collection {
    type Params = SelectAllParams;
    async fn select_all_with_params<'e, E>(
        executor: E,
        params: Self::Params,
    ) -> Result<Vec<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            Collection,
            "  SELECT id, title, slug, description
  FROM app.collections c
  WHERE EXISTS (
      SELECT 1
      FROM app.project_features f
      WHERE f.collection_id = c.id 
      AND f.project_id =  $1
  )
  ORDER BY id",
            params.project_id.0
        )
        .fetch_all(executor)
        .await
        .map_err(RepositoryError::from)
    }
}
