use crate::repo::traits::{SelectAll, SelectBySlug};
use futures::Stream;
use geojson::FeatureCollection;
use sqlx::PgPool;
use sqlx::types::Json;

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

    #[tracing::instrument(skip(self, slug))]
    pub async fn select_by_slug<T>(&self, slug: &str) -> Result<Option<T>, sqlx::Error>
    where
        T: SelectBySlug,
    {
        T::select_by_slug(&self.db_pool, slug).await
    }

    #[tracing::instrument(skip(self, collection_id, limit))]
    pub async fn select_features(
        &self,
        collection_id: i32,
        limit: Option<usize>,
    ) -> Result<FeatureCollection, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT jsonb_build_object(
                'type', 'Feature',
                'id', id,
                'geometry', ST_AsGeoJSON(ST_Transform(geom, 4326))::jsonb,
                'properties', properties || jsonb_build_object('name', name, 'is_primary', is_primary)
            ) as feature
            FROM app.features
            WHERE collection_id = $1 AND status = 'ACTIVE'
            ORDER BY id
            LIMIT $2
            "#,
            collection_id,
            limit.map(|l| l as i64)
        )
        .fetch_all(&self.db_pool)
        .await?;

        let features: Vec<geojson::Feature> = rows
            .into_iter()
            .map(|row| {
                serde_json::from_value(row.feature.unwrap())
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let feature_collection = FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        };

        Ok(feature_collection)
    }

    #[tracing::instrument(skip(self, collection_id, limit))]
    pub fn select_features_streaming(
        &self,
        collection_id: i32,
        limit: Option<usize>,
    ) -> impl Stream<Item = Result<Json<geojson::Feature>, sqlx::Error>> + '_ {
        sqlx::query_scalar!(
            r#"
            SELECT ST_AsGeoJSON(t.*, id_column => 'id')::jsonb as "f!: Json<geojson::Feature>"
            FROM (
                SELECT id, name, ST_Transform(geom, 4326) as geom
                FROM app.features
                WHERE collection_id = $1 AND status = 'ACTIVE'
                ORDER BY id
                LIMIT $2 
                ) 
            as t(id, name, geom)
            "#,
            collection_id,
            limit.map(|l| l as i64)
        )
        .fetch(&self.db_pool)
    }

    #[tracing::instrument(skip(self, collection_id, feature_id))]
    pub async fn select_feature(
        &self,
        collection_id: i32,
        feature_id: i32,
    ) -> Result<Option<geojson::Feature>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT jsonb_build_object(
                'type', 'Feature',
                'id', id,
                'geometry', ST_AsGeoJSON(ST_Transform(geom, 4326))::jsonb,
                'properties',  properties || jsonb_build_object('name', name, 'is_primary', is_primary) 
            ) as feature
            FROM app.features
            WHERE collection_id = $1 AND id = $2 AND status = 'ACTIVE'
            "#,
            collection_id,
            feature_id
        )
        .fetch_optional(&self.db_pool)
        .await?;

        match result {
            Some(row) => {
                let feature: geojson::Feature = serde_json::from_value(row.feature.unwrap())
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
                Ok(Some(feature))
            }
            None => Ok(None),
        }
    }
}
