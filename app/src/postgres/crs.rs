use ogcapi_types::common::Crs;

use crate::postgres::traits::SelectAll;

impl SelectAll for Crs {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, crate::errors::RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let mut supported_crs = vec![Crs::default()];
        for srid in sqlx::query_scalar!(r#"SELECT srid FROM app.supported_crs"#)
            .fetch_all(executor)
            .await?
            .into_iter()
        {
            supported_crs.push(Crs::from_srid(srid))
        }
        Ok(supported_crs)
    }
}
