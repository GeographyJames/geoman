use crate::common::{constants::REQUEST_FAILED, services::HttpClient};
use app::{URLS, enums::ProjectIdentifier};
use domain::{ProjectCollectionId, enums};
use reqwest::{RequestBuilder, Response};
use serde::Serialize;

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

    pub async fn get_collection(
        &self,
        client: &HttpClient,
        collection_id: enums::Collection,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection_id
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_collection(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_id: ProjectCollectionId,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}/{}",
            URLS.ogc_api.base,
            &URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
            collection_id
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_features(
        &self,
        client: &HttpClient,
        collection_id: enums::Collection,
    ) -> Response {
        let req = self.get_features_req(client, collection_id);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_features_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection_id: enums::Collection,
        params: &T,
    ) -> Response {
        let req = self.get_features_req(client, collection_id).query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_features_req(
        &self,
        client: &HttpClient,
        collection_id: enums::Collection,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}/items",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection_id
        ))
    }

    pub async fn get_project_features(
        &self,
        client: &HttpClient,
        collection_id: ProjectCollectionId,
        project: &ProjectIdentifier,
    ) -> Response {
        let req = self.get_project_features_req(client, collection_id, project);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_project_features_req(
        &self,
        client: &HttpClient,
        collection_id: ProjectCollectionId,
        project: &ProjectIdentifier,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}{}/{}/items",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
            collection_id
        ))
    }

    pub async fn get_project_features_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection_id: ProjectCollectionId,
        project: &ProjectIdentifier,
        params: &T,
    ) -> Response {
        let req = self
            .get_project_features_req(client, collection_id, project)
            .query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_feature(
        &self,
        client: &HttpClient,
        collection: &domain::enums::Collection,
        id: i32,
    ) -> Response {
        let req = self.get_feature_req(client, collection, id);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_feature_req(
        &self,
        client: &HttpClient,
        collection: &domain::enums::Collection,
        id: i32,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}/items/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection, id
        ))
    }

    pub async fn get_feature_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection: &domain::enums::Collection,
        id: i32,
        params: &T,
    ) -> Response {
        let req = self.get_feature_req(client, collection, id).query(params);
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
        collection_id: ProjectCollectionId,
        id: i32,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}{}/{}/items/{}",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project,
            URLS.ogc_api.collections,
            collection_id,
            id
        ))
    }

    pub async fn get_project_feature_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_id: ProjectCollectionId,
        id: i32,
        params: &T,
    ) -> Response {
        let req = self
            .get_project_feature_req(client, project, collection_id, id)
            .query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_feature(
        &self,
        client: &HttpClient,
        project: &ProjectIdentifier,
        collection_id: ProjectCollectionId,
        id: i32,
    ) -> Response {
        let req = self.get_project_feature_req(client, project, collection_id, id);
        req.send().await.expect(REQUEST_FAILED)
    }
}
