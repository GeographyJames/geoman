use crate::{
    features::data_providers::types::{DataProvider, DataProviderId},
    repo::traits::SelectAll,
};

impl SelectAll for DataProvider {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        let res = sqlx::query_as!(
            DataProvider,
            r#"
            SELECT id as "id: DataProviderId",
                   name,
                   country_code,
                   subdivision
            FROM app.data_providers
            ORDER BY name ASC
            "#
        )
        .fetch_all(executor)
        .await?;
        Ok(res)
    }
}
