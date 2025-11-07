use geoman::{app::URLS, domain::FeatureId};
use reqwest::Response;

use crate::app::{constants::REQUEST_FAILED, services::HttpClient};

pub struct OgcService {}

impl OgcService {
    pub async fn get_landing_page(&self, client: &HttpClient) -> Response {
        let req = client.get(&URLS.ogc_api.base);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_conformance_declaration(&self, client: &HttpClient) -> Response {
        let req = client.get(format!(
            "{}{}",
            &URLS.ogc_api.base, &URLS.ogc_api.conformance_declaration
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_collections(&self, client: &HttpClient) -> Response {
        let req = client.get(format!(
            "{}{}",
            &URLS.ogc_api.base, &URLS.ogc_api.collections
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_collection(&self, client: &HttpClient, collection_slug: &str) -> Response {
        let req = client.get(format!(
            "{}{}/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection_slug
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_features(&self, client: &HttpClient, collection_slug: &str) -> Response {
        let req = client.get(format!(
            "{}{}/{}/items",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection_slug
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_feature(
        &self,
        client: &HttpClient,
        collection_slug: &str,
        id: FeatureId,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}/items/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection_slug, id.0
        ));
        req.send().await.expect(REQUEST_FAILED)
    }
}
