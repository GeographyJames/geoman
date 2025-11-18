use app::{URLS, enums::ProjectIdentifier};

use domain::Slug;
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

    pub async fn get_project_collections(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}",
            &URLS.ogc_api.base, &URLS.ogc_api.project, project, &URLS.ogc_api.collections
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

    pub async fn get_project_collection(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_slug: &Slug,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}/{}",
            URLS.ogc_api.base,
            &URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
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

    pub async fn get_project_features(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        project: &ProjectIdentifier,
        params: Option<&Query>,
    ) -> Response {
        let endopint = format!(
            "{}{}/{}{}/{}/items",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
            collection_slug.as_ref()
        );
        let mut req = client.get(endopint);
        if let Some(query) = params {
            req = req.query(query)
        }
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_feature(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        id: i32,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}/items/{}",
            &URLS.ogc_api.base,
            &URLS.ogc_api.collections,
            collection_slug.as_ref(),
            id
        ));

        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_feature(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_slug: &Slug,
        id: i32,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}/{}/items/{}",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
            collection_slug.as_ref(),
            id
        ));
        req.send().await.expect(REQUEST_FAILED)
    }
}
