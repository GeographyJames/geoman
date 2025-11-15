use crate::{
    constants::DB_QUERY_FAIL,
    errors::ApiError,
    postgres::{PostgresRepo, traits::SelectAllWithParamsStreaming},
};
use actix_web::web::{self, Bytes};
use anyhow::Context;
use domain::IntoOGCFeature;
use futures::{Stream, StreamExt};
use ogc::types::FeatureCollection;

fn ogc_feature_byte_stream<T, F>(
    stream: T,
    collection_url: String,
) -> impl Stream<Item = Result<Bytes, ApiError>>
where
    T: Stream<Item = Result<F, sqlx::Error>>,
    F: IntoOGCFeature,
{
    stream.enumerate().map(move |(index, res)| {
        res.context(DB_QUERY_FAIL)
            .map_err(ApiError::from)
            .and_then(|feature_row| {
                let feature = feature_row.into_ogc_feature(collection_url.clone());
                let mut bytes = if index == 0 { Vec::new() } else { vec![b','] };
                serde_json::to_writer(&mut bytes, &feature)
                    .context("Failed to serialise feature to Json")?;
                Ok(Bytes::from(bytes))
            })
    })
}

pub fn feature_collection_byte_stream<T>(
    repo: web::Data<PostgresRepo>,
    params: T::Params,
    collection_url: String,
    collection_id: String,
) -> Result<impl Stream<Item = Result<Bytes, ApiError>>, ApiError>
where
    T: SelectAllWithParamsStreaming + IntoOGCFeature,
{
    let feature_collection = FeatureCollection::new(collection_url.clone(), collection_id);

    let opening_json = feature_collection
        .opening_json()
        .context("failed to deserialise feature collection opening json")?;
    let opening_stream = futures::stream::once(async move { Bytes::from(opening_json) });

    let database_stream = repo.as_ref().select_all_with_params_streaming::<T>(params);
    let feature_stream = ogc_feature_byte_stream(database_stream, collection_url);

    let closing_json = feature_collection.closing_json();
    let closing_stream = futures::stream::once(async move { Bytes::from(closing_json) });

    Ok(opening_stream
        .map(Ok)
        .chain(feature_stream)
        .chain(closing_stream.map(Ok)))
}
