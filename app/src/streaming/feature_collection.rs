use std::sync::{Arc, Mutex};

use crate::{
    errors::{ApiError, RepositoryError},
    postgres::StreamItem,
};
use actix_web::web::Bytes;
use anyhow::Context;
use domain::IntoOGCFeature;
use futures::{Stream, StreamExt, stream};

fn ogc_feature_byte_stream<T, F>(
    stream: T,
    collection_url: String,
) -> impl Stream<Item = Result<(Bytes, usize), ApiError>>
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
            Ok((Bytes::from(bytes), index))
        })
    })
}

pub async fn ogc_feature_collection_byte_stream<T, S>(
    mut database_stream: S,
    collection_url: String,
    collection_id: String,
) -> Result<impl Stream<Item = Result<Bytes, ApiError>>, ApiError>
where
    S: Stream<Item = Result<StreamItem<T>, RepositoryError>> + Unpin,
    T: IntoOGCFeature,
{
    // Check first item for database error and return early if it fails
    let first_item = database_stream.next().await.transpose()?;
    let number_matched = first_item
        .as_ref()
        .map(|item: &StreamItem<T>| item.total_count)
        .unwrap_or(0);
    let feature_items = stream::iter(first_item.into_iter().map(Ok))
        .chain(database_stream)
        .map(|res| res.map(|stream_item| stream_item.item));

    let feature_collection = ogc::FeatureCollection::new(collection_url.clone(), collection_id);

    let opening_json = feature_collection
        .opening_json(number_matched)
        .context("failed to deserialise feature collection opening json")?;
    let opening_stream = futures::stream::once(async move { Bytes::from(opening_json) });

    let feature_stream_with_index = ogc_feature_byte_stream(feature_items, collection_url);
    let last_index = Arc::new(Mutex::new(None));
    let last_index_clone = last_index.clone();

    let feature_stream = feature_stream_with_index.map(move |res| {
        res.map(|(bytes, index)| {
            *last_index_clone.lock().unwrap() = Some(index);
            bytes
        })
    });

    let closing_stream = futures::stream::once(async move {
        let number_returned = last_index.lock().unwrap().map(|idx| idx + 1).unwrap_or(0);

        let closing_json = feature_collection.closing_json(number_returned).unwrap();
        Bytes::from(closing_json)
    });

    Ok(opening_stream
        .map(Ok)
        .chain(feature_stream)
        .chain(closing_stream.map(Ok)))
}
