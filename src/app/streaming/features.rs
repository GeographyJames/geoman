use crate::{
    app::errors::ApiError,
    constants::DB_QUERY_FAIL,
    ogc,
    repo::{PostgresRepo, postgres::features::SelectAllParams},
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
                    let feature = ogc::types::Feature::from_feature_row(
                        feature_row,
                        collection_url.to_string(),
                    );
                    let mut bytes = if index == 0 { Vec::new() } else { vec![b','] };
                    serde_json::to_writer(&mut bytes, &feature)
                        .context("Failed to serialise feature to Json")?;
                    Ok(Bytes::from(bytes))
                })
        })
}

fn opening_json(slug: &str, collection_url: &str) -> String {
    format!(
        r#"{{"type":"FeatureCollection","id":"{}","links":{},"features":["#,
        slug,
        serde_json::to_string(&[ogc::types::common::Link::new(
            format!("{}/items", collection_url),
            ogc::types::common::link_relations::SELF
        )
        .mediatype(ogc::types::common::media_types::MediaType::GeoJson)])
        .unwrap()
    )
}

fn opening_stream(slug: String, collection_url: String) -> impl Stream<Item = Bytes> {
    futures::stream::once(async move { Bytes::from(opening_json(&slug, &collection_url)) })
}

fn closing_stream() -> impl Stream<Item = Bytes> {
    futures::stream::once(async move { Bytes::from("]}") })
}

pub fn features_byte_stream(
    repo: web::Data<PostgresRepo>,
    params: SelectAllParams,
    collection_url: String,
) -> impl Stream<Item = Result<Bytes, ApiError>> {
    opening_stream(params.slug.clone(), collection_url.clone())
        .map(Ok)
        .chain(feature_stream(repo, params, collection_url))
        .chain(closing_stream().map(Ok))
}
