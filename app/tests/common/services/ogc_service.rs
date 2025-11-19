use app::{URLS, enums::ProjectIdentifier};

use domain::Slug;
use ogc::features::Query;
use reqwest::{RequestBuilder, Response};
use serde::Serialize;

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

    pub async fn get_features(&self, client: &HttpClient, collection_slug: &Slug) -> Response {
        let req = self.get_features_req(client, collection_slug);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_features_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        params: &T,
    ) -> Response {
        let req = self.get_features_req(client, collection_slug).query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_features_req(&self, client: &HttpClient, collection_slug: &Slug) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}/items",
            &URLS.ogc_api.base,
            &URLS.ogc_api.collections,
            collection_slug.as_ref()
        ))
    }

    pub async fn get_project_features(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        project: &ProjectIdentifier,
    ) -> Response {
        let req = self.get_project_features_req(client, collection_slug, project);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_project_features_req(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        project: &ProjectIdentifier,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}{}/{}/items",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
            collection_slug.as_ref()
        ))
    }

    pub async fn get_project_features_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        project: &ProjectIdentifier,
        params: &T,
    ) -> Response {
        let req = self
            .get_project_features_req(client, collection_slug, project)
            .query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_feature(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        id: i32,
    ) -> Response {
        let req = self.get_feature_req(client, collection_slug, id);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_feature_req(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        id: i32,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}/items/{}",
            &URLS.ogc_api.base,
            &URLS.ogc_api.collections,
            collection_slug.as_ref(),
            id
        ))
    }

    pub async fn get_feature_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection_slug: &Slug,
        id: i32,
        params: &T,
    ) -> Response {
        let req = self
            .get_feature_req(client, collection_slug, id)
            .query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_openapi(&self, client: &HttpClient) -> Response {
        let req = client.get(format!("{}{}", URLS.ogc_api.base, URLS.ogc_api.openapi));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_openapi(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}",
            URLS.ogc_api.base, URLS.ogc_api.project, project, URLS.ogc_api.openapi
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_project_feature_req(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_slug: &Slug,
        id: i32,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}{}/{}/items/{}",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
            collection_slug.as_ref(),
            id
        ))
    }

    pub async fn get_project_feature_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_slug: &Slug,
        id: i32,
        params: &T,
    ) -> Response {
        let req = self
            .get_project_feature_req(client, project, collection_slug, id)
            .query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_feature(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_slug: &Slug,
        id: i32,
    ) -> Response {
        let req = self.get_project_feature_req(client, project, collection_slug, id);
        req.send().await.expect(REQUEST_FAILED)
    }
}
