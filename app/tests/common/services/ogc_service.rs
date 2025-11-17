use app::{URLS, enums::ProjectIdentifier};

use domain::{FeatureId, Slug};
use ogc::features::Query;
use reqwest::Response;

use crate::common::{constants::REQUEST_FAILED, services::HttpClient};

pub struct OgcService {}

impl OgcService {
    pub async fn get_landing_page(&self, client: &HttpClient) -> Response {
        let req = client.get(&URLS.ogc_api.base);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_landing_page(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.project, project
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_conformance_declaration(&self, client: &HttpClient) -> Response {
        let req = client.get(format!(
            "{}{}",
            &URLS.ogc_api.base, &URLS.ogc_api.conformance_declaration
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_conformance_declaration(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}",
            &URLS.ogc_api.base,
            &URLS.ogc_api.project,
            project,
            &URLS.ogc_api.conformance_declaration
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

    pub async fn get_collection(&self, client: &HttpClient, collection_slug: &Slug) -> Response {
        let req = client.get(format!(
            "{}{}/{}",
            &URLS.ogc_api.base,
            &URLS.ogc_api.collections,
            collection_slug.as_ref()
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_features(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        params: Option<&Query>,
    ) -> Response {
        let mut req = client.get(format!(
            "{}{}/{}/items",
            &URLS.ogc_api.base,
            &URLS.ogc_api.collections,
            collection_slug.as_ref()
        ));
        if let Some(query) = params {
            req = req.query(query)
        }
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_feature(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        id: FeatureId,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}/items/{}",
            &URLS.ogc_api.base,
            &URLS.ogc_api.collections,
            collection_slug.as_ref(),
            id.0
        ));
        req.send().await.expect(REQUEST_FAILED)
    }
}
