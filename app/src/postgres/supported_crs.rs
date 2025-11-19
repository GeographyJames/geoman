use domain::SupportedCrs;

use crate::{errors::RepositoryError, postgres::traits::SelectAll};

impl SelectAll for SupportedCrs {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, crate::errors::RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            SupportedCrs,
            r#"SELECT scrs.srid, auth_name as "auth_name!" FROM app.supported_crs scrs JOIN spatial_ref_sys srs ON scrs.srid = srs.srid "#
        ).fetch_all(executor).await.map_err(RepositoryError::from)
    }
}
