/// Appplication repository
use domain::{KeyId, LayoutId, ProjectCollectionId, ProjectId, UserId, enums::GeometryType, name::NameInputDTO};
use futures::Stream;
use sqlx::PgPool;

use crate::repo::{
    PoolWrapper, RepositoryError, StreamItem,
    traits::{
        Insert, SelectAll, SelectAllWithParams, SelectAllWithParamsStreaming, SelectOne,
        SelectOneWithParams, Update,
    },
};

pub struct PostgresRepo {
    pub db_pool: PgPool,
}

impl PostgresRepo {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        use sqlx::postgres::PgPoolOptions;
        use std::time::Duration;

        let db_pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(1)) // Timeout immediately when trying to get a connection
            .idle_timeout(Duration::from_millis(1)) // Close idle connections immediately
            .connect_lazy("postgres://unused:unused@localhost/unused")
            .expect("failed to create mock postgres pool");
        Self { db_pool }
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_all<T>(&self) -> Result<Vec<T>, RepositoryError>
    where
        T: SelectAll,
    {
        T::select_all(&self.db_pool).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn select_one<T, ID>(&self, id: ID) -> Result<Option<T>, RepositoryError>
    where
        T: SelectOne<ID>,
    {
        T::select_one(&self.db_pool, id).await
    }

    #[tracing::instrument(skip(self, params))]
    pub fn select_all_with_params_streaming<'a, T>(
        &self,
        params: T::Params<'a>,
    ) -> impl Stream<Item = Result<StreamItem<T>, RepositoryError>> + use<T>
    where
        T: SelectAllWithParamsStreaming,
    {
        let executor = PoolWrapper(self.db_pool.clone());
        T::select_all_with_params_streaming(executor, params)
    }

    #[tracing::instrument(skip(self, params))]
    pub async fn select_all_with_params<'a, T>(
        &'a self,
        params: T::Params<'a>,
    ) -> Result<(Vec<T>, T::MetaData<'a>), RepositoryError>
    where
        T: SelectAllWithParams,
    {
        T::select_all_with_params(&self.db_pool, params).await
    }

    #[tracing::instrument(skip(self, params, id))]
    pub async fn select_one_with_params<'a, T, ID>(
        &'a self,
        id: ID,
        params: T::Params<'a>,
    ) -> Result<Option<T>, RepositoryError>
    where
        T: SelectOneWithParams<ID>,
    {
        T::select_one_with_params(&self.db_pool, id, params).await
    }

    #[tracing::instrument(skip(self, item))]
    pub async fn insert<T>(&self, item: &T) -> Result<T::Id, RepositoryError>
    where
        T: Insert,
    {
        item.insert(&self.db_pool).await
    }
    #[tracing::instrument(skip(self, item))]
    pub async fn update<T>(&self, item: &T) -> Result<T::Id, RepositoryError>
    where
        T: Update,
    {
        item.update(&self.db_pool).await
    }

    #[tracing::instrument(skip(self, id, user_id))]
    pub async fn revoke_api_key(&self, id: KeyId, user_id: UserId) -> Result<(), RepositoryError> {
        sqlx::query_scalar!(
            "
        UPDATE app.api_keys
        SET revoked = NOW()
        WHERE id = $1
        AND user_id = $2
        RETURNING id",
            id.0,
            user_id.0
        )
        .fetch_one(&self.db_pool)
        .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id, user_id))]
    pub async fn renew_api_key(&self, id: KeyId, user_id: UserId) -> Result<(), RepositoryError> {
        sqlx::query_scalar!(
            "
        UPDATE app.api_keys
        SET expiry = (NOW() + INTERVAL '6 months')
        WHERE id = $1
        AND user_id =  $2
        RETURNING id",
            id.0,
            user_id.0
        )
        .fetch_one(&self.db_pool)
        .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_collection_geom_type(
        &self,
        id: ProjectCollectionId,
    ) -> Result<GeometryType, RepositoryError> {
        let geom = sqlx::query_scalar!(
            r#"SELECT geometry_type as "geometry_type: GeometryType" FROM app.collections WHERE id = $1"#,
            id.0
        )
        .fetch_one(&self.db_pool)
        .await?;
        Ok(geom)
    }
    #[tracing::instrument(skip(self, id))]
    pub async fn get_project_srid(&self, id: ProjectId) -> Result<Option<i32>, RepositoryError> {
        let srid = sqlx::query_scalar!("SELECT crs_srid FROM app.projects WHERE id = $1", id.0)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(srid)
    }

    #[tracing::instrument(skip(self, name))]
    pub async fn duplicate_turbine_layout(
        &self,
        project_id: ProjectId,
        source_layout_id: LayoutId,
        user_id: UserId,
        name: Option<NameInputDTO>,
        hub_height_mm: Option<i32>,
        rotor_diameter_mm: Option<i32>,
        primary: Option<bool>,
    ) -> Result<LayoutId, RepositoryError> {
        super::turbine_layouts::duplicate::duplicate_turbine_layout(
            &self.db_pool,
            project_id,
            source_layout_id,
            user_id,
            name,
            hub_height_mm,
            rotor_diameter_mm,
            primary,
        )
        .await
    }
}
