use crate::repo::{api_keys::SelectAllParams, traits::SelectAllWithParams};
use domain::ApiKey;
use domain::KeyId;
use std::net::IpAddr;

impl SelectAllWithParams for ApiKey {
    type Params<'a> = SelectAllParams;

    type MetaData<'a> = ();

    async fn select_all_with_params<'a, E>(
        executor: &'a E,
        params: Self::Params<'a>,
    ) -> Result<(Vec<Self>, Self::MetaData<'a>), crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        let keys = sqlx::query_as!(
            ApiKey,
            r#"
        SELECT k.id as "id: KeyId",
               k.name,
               k.created,
               k.last_used,
               k.expiry,
               k.last_used_ip as "last_used_ip:IpAddr",
               k.last_used_user_agent
          FROM app.api_keys k
          JOIN app.users u ON k.user_id = u.id
         WHERE k.revoked IS NULL
           AND u.id = $1
         ORDER BY k.created DESC
           "#,
            params.user_id.0
        )
        .fetch_all(executor)
        .await?;
        Ok((keys, ()))
    }
}
