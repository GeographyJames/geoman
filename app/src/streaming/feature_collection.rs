use std::sync::{Arc, Mutex};

use actix_web::web::Bytes;
use anyhow::Context;
use domain::{IntoOGCFeature, enums::CollectionId};
use futures::{Stream, StreamExt, stream};

use crate::{
    handlers::ogc_api::features::Query,
    repo::{RepositoryError, StreamItem},
};

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
    query: Query,
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

    let opening_json = ogc::FeatureCollection::opening_json(&collection_url, number_matched)
        .context("failed to deserialise feature collection opening json")?;
    let opening_stream = futures::stream::once(async move { Bytes::from(opening_json) });

    let feature_stream_with_index =
        ogc_feature_byte_stream(feature_items, collection_id.to_string());
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
        let next_url = if let Some(l) = query.limit
            && number_returned == l
            && (query.offset.unwrap_or(0) + number_returned) < number_matched as usize
        {
            next_url(&query, &collection_url)?
        } else {
            None
        };

        let closing_json = ogc::FeatureCollection::closing_json(
            &collection_url,
            number_returned,
            next_url.as_deref(),
        )
        .context("failed to serialise feature closing json")?;
        Ok(Bytes::from(closing_json))
    });

    Ok(opening_stream
        .map(Ok)
        .chain(feature_stream)
        .chain(closing_stream))
}

fn next_url(query: &Query, collection_url: &str) -> Result<Option<String>, anyhow::Error> {
    let next_url = query
        .limit
        .map(|current_limit| {
            let current_offset = query.offset.unwrap_or(0);
            let next_offset = current_offset + current_limit;
            let mut next_query = query.clone();
            next_query.offset = Some(next_offset);

            serde_urlencoded::to_string(next_query)
                .map(|qs| format!("{}/items?{}", collection_url, qs))
        })
        .transpose()
        .context("failed to generate 'next' link")?;
    Ok(next_url)
}
