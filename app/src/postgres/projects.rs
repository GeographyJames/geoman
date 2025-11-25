use domain::{Project, ProjectId, project::Properties};

use crate::repo::{
    RepositoryError,
    metadata::NumberMatched,
    project::SelectAllParams,
    traits::{SelectAllWithParams, SelectOne},
};

pub struct ProjectRow {
    id: i32,
    name: String,
    number_matched: i64,
}

#[allow(clippy::from_over_into)]
impl Into<Project> for ProjectRow {
    fn into(self) -> Project {
        let ProjectRow { id, name, .. } = self;
        let properties = Properties { name };
        Project { id, properties }
    }
}

impl SelectAllWithParams for Project {
    type Params<'a> = SelectAllParams;
    type MetaData<'a> = NumberMatched;
    async fn select_all_with_params<'a, 'e, E>(
        executor: &'e E,
        params: Self::Params<'a>,
    ) -> Result<(Vec<Self>, Self::MetaData<'a>), RepositoryError>
    where
        &'e E: sqlx::PgExecutor<'e>,
    {
        let rows = sqlx::query_as!(
                ProjectRow,
                r#"SELECT id, name, COUNT(*) OVER() as "number_matched!" FROM app.projects ORDER BY id LIMIT $1"#,
                params.limit.map(|l| l as i64)
            )
            .fetch_all(executor)
            .await?;
        let number_matched = rows.first().map(|item| item.number_matched).unwrap_or(0);

        let items = rows
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<Project>>();
        Ok((items, NumberMatched(number_matched)))
    }
}

impl SelectOne for Project {
    type Id<'a> = ProjectId;
    async fn select_one<'a, 'e, E>(
        executor: &'e E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'e E: sqlx::PgExecutor<'e>,
    {
        let project_row = sqlx::query_as!(
            ProjectRow,
            r#"SELECT id, name, 1 as "number_matched!" FROM app.projects WHERE id = $1"#,
            id.0
        )
        .fetch_optional(executor)
        .await?;

        Ok(project_row.map(|row| row.into()))
    }
}
