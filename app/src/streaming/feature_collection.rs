use std::sync::{Arc, Mutex};

use actix_web::web::Bytes;
use anyhow::Context;
use domain::{IntoOGCFeature, enums::CollectionId};
use futures::{Stream, StreamExt, stream};

use crate::repo::{RepositoryError, StreamItem};

fn ogc_feature_byte_stream<T, F>(
    stream: T,
    collection_url: String,
) -> impl Stream<Item = Result<(Bytes, usize), anyhow::Error>>
where
    T: Stream<Item = Result<F, RepositoryError>>,
    F: IntoOGCFeature,
{
    stream.enumerate().map(move |(index, res)| {
        res.map_err(Into::into).and_then(|feature_row| {
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
    collection_id: CollectionId,
) -> Result<impl Stream<Item = Result<Bytes, anyhow::Error>>, anyhow::Error>
where
    S: Stream<Item = Result<StreamItem<T>, RepositoryError>> + Unpin,
    T: IntoOGCFeature,
{
    // Check first item for database error and return early if it fails
    let first_item = database_stream.next().await.transpose()?;
    let number_matched = first_item
        .as_ref()
        .map(|item: &StreamItem<T>| item.number_matched)
        .unwrap_or(0);
    let feature_items = stream::iter(first_item.into_iter().map(Ok))
        .chain(database_stream)
        .map(|res| res.map(|stream_item| stream_item.item));

    let opening_json = ogc::FeatureCollection::opening_json(
        &collection_id.to_string(),
        &collection_url,
        number_matched,
    )
    .context("failed to deserialise feature collection opening json")?;
    let opening_stream = futures::stream::once(async move { Bytes::from(opening_json) });

    let feature_stream_with_index = ogc_feature_byte_stream(feature_items, collection_url);
    let last_index = Arc::new(Mutex::new(None));
    let last_index_clone = last_index.clone();

    let feature_stream = feature_stream_with_index.map(move |res| {
        res.and_then(|(bytes, index)| {
            *last_index_clone
                .lock()
                .map_err(|e| anyhow::anyhow!("Mutex poisoned: {}", e))? = Some(index);
            Ok(bytes)
        })
    });

    let closing_stream = futures::stream::once(async move {
        let number_returned = last_index
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex poisoned: {}", e))?
            .map(|idx| idx + 1)
            .unwrap_or(0);

        let closing_json = ogc::FeatureCollection::closing_json(number_returned)
            .context("failed to serialise feature closing json")?;
        Ok(Bytes::from(closing_json))
    });

    Ok(opening_stream
        .map(Ok)
        .chain(feature_stream)
        .chain(closing_stream))
}
