use crate::{
    app::errors::ApiError,
    constants::DB_QUERY_FAIL,
    ogc::{self, types::FeatureCollection},
    postgres::{PostgresRepo, ogc::features::SelectAllParams},
};
use actix_web::web::{self, Bytes};
use anyhow::Context;
use futures::{Stream, StreamExt};

fn feature_stream(
    repo: web::Data<PostgresRepo>,
    params: SelectAllParams,
    collection_url: String,
) -> impl Stream<Item = Result<Bytes, ApiError>> {
    repo.as_ref()
        .select_all_with_params_streaming(params)
        .enumerate()
        .map(move |(index, res)| {
            res.context(DB_QUERY_FAIL)
                .map_err(ApiError::from)
                .and_then(|feature_row| {
                    let feature =
                        ogc::types::Feature::from_feature_row(feature_row, collection_url.clone());
                    let mut bytes = if index == 0 { Vec::new() } else { vec![b','] };
                    serde_json::to_writer(&mut bytes, &feature)
                        .context("Failed to serialise feature to Json")?;
                    Ok(Bytes::from(bytes))
                })
        })
}

pub fn feature_collection_byte_stream(
    repo: web::Data<PostgresRepo>,
    params: SelectAllParams,
    collection_url: String,
) -> Result<impl Stream<Item = Result<Bytes, ApiError>>, ApiError> {
    let feature_collection = FeatureCollection::new(collection_url.clone(), params.slug.clone());
    let opening_json = feature_collection
        .opening_json()
        .context("failed to deserialise feature collection opening json")?;
    let closing_json = feature_collection.closing_json();
    let opening_stream = futures::stream::once(async move { Bytes::from(opening_json) });
    let closing_stream = futures::stream::once(async move { Bytes::from(closing_json) });
    Ok(opening_stream
        .map(Ok)
        .chain(feature_stream(repo, params, collection_url))
        .chain(closing_stream.map(Ok)))
}
