use domain::Project;
use futures::{Stream, StreamExt};

use crate::{
    enums::ProjectIdentifier,
    errors::RepositoryError,
    postgres::{
        PoolWrapper,
        traits::{SelectAllStreaiming, SelectOne},
    },
};

impl SelectAllStreaiming for Project {
    fn select_all_streaming(
        executor: PoolWrapper,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<> {
        sqlx::query_as!(Project, "SELECT id, name, slug FROM app.projects")
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
