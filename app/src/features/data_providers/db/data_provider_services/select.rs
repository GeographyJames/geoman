use crate::{
    features::data_providers::types::{
        DataProviderId, DataProviderService, DataProviderServiceId, DataProviderServiceType,
    },
    repo::traits::SelectAll,
};

impl SelectAll for DataProviderService {
    async fn select_all<'a, A>(executor: A) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let res = sqlx::query_as!(
            DataProviderService,
            r#"
            SELECT id as "id: DataProviderServiceId",
                   provider_id as "provider_id: DataProviderId",
                   name,
                   service_type as "service_type: DataProviderServiceType",
                   base_url
            FROM app.data_provider_services
            ORDER BY name ASC
            "#
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(res)
    }
}
