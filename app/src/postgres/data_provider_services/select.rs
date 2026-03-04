use domain::enums::DataProviderServiceType;
use domain::{DataProviderId, DataProviderService, DataProviderServiceId};

use crate::repo::traits::SelectAll;

impl SelectAll for DataProviderService {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
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
        .fetch_all(executor)
        .await?;
        Ok(res)
    }
}
