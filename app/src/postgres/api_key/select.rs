use crate::repo::{api_keys::SelectAllParams, traits::SelectAllWithParams};
use domain::ApiKey;
use domain::KeyId;
use std::net::IpAddr;

impl SelectAllWithParams for ApiKey {
    type Params<'a> = SelectAllParams<'a>;

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
        SELECT keys.id as "id: KeyId",
               name,
               created,
               last_used,
               expiry,
               last_used_ip as "last_used_ip:IpAddr",
               last_used_user_agent
          FROM app.api_keys keys
          JOIN app.users users ON users.id = keys.user_id
         WHERE revoked IS NULL
           AND users.clerk_id = $1
         ORDER BY created DESC
           "#,
            params.clerk_id
        )
        .fetch_all(executor)
        .await?;
        Ok((keys, ()))
    }
}
