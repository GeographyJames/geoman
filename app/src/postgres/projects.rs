use domain::{Project, ProjectId, project::Properties};

use crate::{
    errors::RepositoryError,
    handlers::ogc_api::features::Query,
    postgres::traits::{SelectAllWithParams, SelectOne},
};

#[derive(Default)]
pub struct SelectAllParams {
    pub limit: Option<usize>,
}

#[allow(clippy::from_over_into)]
impl Into<Project> for ProjectRow {
    fn into(self) -> Project {
        let ProjectRow { id, name } = self;
        let properties = Properties { name };
        Project { id, properties }
    }
}

pub struct ProjectRow {
    id: i32,
    name: String,
}

impl From<Query> for SelectAllParams {
    fn from(value: Query) -> Self {
        let Query { limit, .. } = value;
        Self { limit }
    }
}

impl SelectAllWithParams for Project {
    type Params<'a> = SelectAllParams;
    async fn select_all_with_params<'a, 'e, E>(
        executor: E,
        params: Self::Params<'a>,
    ) -> Result<Vec<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        Ok(sqlx::query_as!(
            ProjectRow,
            "SELECT id, name FROM app.projects ORDER BY id LIMIT $1",
            params.limit.map(|l| l as i64)
        )
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(|row| row.into())
        .collect())
    }
}

impl SelectOne for Project {
    type Id<'a> = ProjectId;
    async fn select_one<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let project_row = sqlx::query_as!(
            ProjectRow,
            "SELECT id, name FROM app.projects WHERE id = $1",
            id.0
        )
        .fetch_optional(executor)
        .await?;

        Ok(project_row.map(|row| row.into()))
    }
}
