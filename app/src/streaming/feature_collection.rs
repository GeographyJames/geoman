use crate::errors::{ApiError, RepositoryError};
use actix_web::web::Bytes;
use anyhow::Context;
use domain::IntoOGCFeature;
use futures::{Stream, StreamExt, stream};

fn ogc_feature_byte_stream<T, F>(
    stream: T,
    collection_url: String,
) -> impl Stream<Item = Result<Bytes, ApiError>>
where
    T: Stream<Item = Result<F, RepositoryError>>,
    F: IntoOGCFeature,
{
    stream.enumerate().map(move |(index, res)| {
        res.map_err(ApiError::from).and_then(|feature_row| {
            let feature = feature_row.into_ogc_feature(collection_url.clone());
            let mut bytes = if index == 0 { Vec::new() } else { vec![b','] };
            serde_json::to_writer(&mut bytes, &feature)
                .context("Failed to serialise feature to Json")?;
            Ok(Bytes::from(bytes))
        })
    })
}

pub async fn ogc_feature_collection_byte_stream<T, S>(
    mut database_stream: S,
    collection_url: String,
    collection_id: String,
) -> Result<impl Stream<Item = Result<Bytes, ApiError>>, ApiError>
where
    S: Stream<Item = Result<T, RepositoryError>> + Unpin,
    T: IntoOGCFeature,
{
    // Check first item for database error and return early if it fails
    let first_item = database_stream.next().await.transpose()?;
    let items = stream::iter(first_item.into_iter().map(Ok)).chain(database_stream);

    let feature_collection = ogc::FeatureCollection::new(collection_url.clone(), collection_id);

    let opening_json = feature_collection
        .opening_json()
        .context("failed to deserialise feature collection opening json")?;
    let opening_stream = futures::stream::once(async move { Bytes::from(opening_json) });

    let feature_stream = ogc_feature_byte_stream(items, collection_url);

    let closing_json = feature_collection.closing_json();
    let closing_stream = futures::stream::once(async move { Bytes::from(closing_json) });

    Ok(opening_stream
        .map(Ok)
        .chain(feature_stream)
        .chain(closing_stream.map(Ok)))
}
