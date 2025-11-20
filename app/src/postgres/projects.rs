use domain::{Project, project::Properties};
use futures::{Stream, StreamExt};

use crate::{
    enums::ProjectIdentifier,
    errors::RepositoryError,
    handlers::ogc_api::features::Query,
    postgres::{
        PoolWrapper,
        traits::{SelectAllWithParamsStreaming, SelectOne},
    },
};

#[derive(Default)]
pub struct SelectAllParams {
    pub limit: Option<usize>,
}

impl Into<Project> for ProjectRow {
    fn into(self) -> Project {
        let ProjectRow { id, name, slug } = self;
        let properties = Properties { slug, name };
        Project { id, properties }
    }
}

pub struct ProjectRow {
    id: i32,
    name: String,
    slug: String,
}

impl From<Query> for SelectAllParams {
    fn from(value: Query) -> Self {
        let Query { limit, .. } = value;
        Self { limit }
    }
}

impl SelectAllWithParamsStreaming for Project {
    type Params<'a> = SelectAllParams;
    fn select_all_with_params_streaming<'a>(
        executor: PoolWrapper,
        params: Self::Params<'a>,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<> {
        sqlx::query_as!(
            ProjectRow,
            "SELECT id, name, slug FROM app.projects ORDER BY id LIMIT $1",
            params.limit.map(|l| l as i64)
        )
        .fetch(executor)
        .map(|res| res.map(|row| row.into()).map_err(RepositoryError::from))
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
        let project_row = match id {
            ProjectIdentifier::Id(id) => {
                sqlx::query_as!(
                    ProjectRow,
                    "SELECT id, name, slug FROM app.projects WHERE id = $1",
                    id.0
                )
                .fetch_optional(executor)
                .await?
            }

            ProjectIdentifier::Slug(slug) => {
                sqlx::query_as!(
                    ProjectRow,
                    "SELECT id, name, slug FROM app.projects WHERE slug = $1",
                    slug
                )
                .fetch_optional(executor)
                .await?
            }
        };
        Ok(project_row.map(|row| row.into()))
    }
}
