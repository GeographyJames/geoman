use crate::repo::{api_keys::SelectAllParams, traits::SelectAllWithParams};
use domain::ApiKey;
use domain::KeyId;

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
               expiry
          FROM app.api_keys keys
          JOIN app.users users ON users.id = keys.user_id
         WHERE revoked = false
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
