use crate::{
    features::data_providers::types::{DataProvider, DataProviderId},
    repo::traits::SelectAll,
};

impl SelectAll for DataProvider {
    async fn select_all<'a, A>(executor: A) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
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
        .fetch_all(&mut *conn)
        .await?;
        Ok(res)
    }
}
