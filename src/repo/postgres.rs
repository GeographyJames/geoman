use geojson::FeatureCollection;
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

    #[tracing::instrument(skip(self))]
    pub async fn select_features(
        &self,
        collection_id: i32,
    ) -> Result<FeatureCollection, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT jsonb_build_object(
                'type', 'FeatureCollection',
                'features', COALESCE(jsonb_agg(
                    jsonb_build_object(
                        'type', 'Feature',
                        'id', id,
                        'geometry', ST_AsGeoJSON(geom)::jsonb,
                        'properties', properties || jsonb_build_object('name', name)
                    )
                ), '[]'::jsonb)
            ) as feature_collection
            FROM app.features
            WHERE collection_id = $1 AND status = 'ACTIVE'
            "#,
            collection_id
        )
        .fetch_one(&self.db_pool)
        .await?;

        let feature_collection: FeatureCollection =
            serde_json::from_value(result.feature_collection.unwrap())
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(feature_collection)
    }

    #[tracing::instrument(skip(self))]
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
                'geometry', ST_AsGeoJSON(geom)::jsonb,
                'properties', properties || jsonb_build_object('name', name)
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
                let feature: geojson::Feature =
                    serde_json::from_value(row.feature.unwrap())
                        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
                Ok(Some(feature))
            }
            None => Ok(None),
        }
    }
}
