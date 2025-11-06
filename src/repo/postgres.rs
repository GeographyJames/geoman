use sqlx::PgPool;

use crate::repo::traits::{SelectAll, SelectBySlug};

pub struct PostgresRepo {
    pub db_pool: PgPool,
}

impl PostgresRepo {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_all<T>(&self) -> Result<Vec<T>, sqlx::Error>
    where
        T: SelectAll,
    {
        T::select_all(&self.db_pool).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_by_slug<T>(&self, slug: &str) -> Result<Option<T>, sqlx::Error>
    where
        T: SelectBySlug,
    {
        T::select_by_slug(&self.db_pool, slug).await
    }
}
