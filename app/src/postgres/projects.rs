use domain::Project;
use futures::{Stream, StreamExt};
use ogc::features::Query;

use crate::{
    enums::ProjectIdentifier,
    errors::RepositoryError,
    postgres::{
        PoolWrapper,
        traits::{SelectAllWithParamsStreaming, SelectOne},
    },
};

#[derive(Default)]
pub struct SelectAllParams {
    limit: Option<usize>,
}

impl From<Query> for SelectAllParams {
    fn from(value: Query) -> Self {
        let Query { limit, .. } = value;
        Self { limit }
    }
}

impl SelectAllWithParamsStreaming for Project {
    type Params = SelectAllParams;
    fn select_all_with_params_streaming(
        executor: PoolWrapper,
        params: Self::Params,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<> {
        sqlx::query_as!(
            Project,
            "SELECT id, name, slug FROM app.projects ORDER BY id LIMIT $1",
            params.limit.map(|l| l as i64)
        )
        .fetch(executor)
        .map(|res| res.map_err(RepositoryError::from))
    }
}

impl SelectOne for Project {
    type Id<'a> = &'a ProjectIdentifier;
    async fn select_one<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let project = match id {
            ProjectIdentifier::Id(id) => {
                sqlx::query_as!(
                    Project,
                    "SELECT id, name, slug FROM app.projects WHERE id = $1",
                    id.0
                )
                .fetch_optional(executor)
                .await?
            }

            ProjectIdentifier::Slug(slug) => {
                sqlx::query_as!(
                    Project,
                    "SELECT id, name, slug FROM app.projects WHERE slug = $1",
                    slug
                )
                .fetch_optional(executor)
                .await?
            }
        };
        Ok(project)
    }
}
