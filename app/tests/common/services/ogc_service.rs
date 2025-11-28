use crate::common::{
    constants::{AUTHORISATION_HEADER, REQUEST_FAILED},
    helpers::handle_json_response,
    services::HttpClient,
    types::SessionToken,
};
use app::URLS;
use domain::{ProjectCollectionId, ProjectId};
use reqwest::{RequestBuilder, Response};
use serde::Serialize;

pub enum OgcAuth {
    Key(String),
    Token(SessionToken),
}

pub struct OgcService {}

impl OgcService {
    pub async fn get_landing_page(&self, client: &HttpClient, auth: Option<OgcAuth>) -> Response {
        let mut req = client.get(&URLS.ogc_api.base);
        if let Some(auth) = auth {
            req = match auth {
                OgcAuth::Key(key) => req.bearer_auth(key),
                OgcAuth::Token(token) => req.header(AUTHORISATION_HEADER, token.jwt),
            }
        };
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_landing_page(
        &self,
        client: &HttpClient,
        project: ProjectId,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.project, project.0
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
        project: ProjectId,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}",
            &URLS.ogc_api.base,
            &URLS.ogc_api.project,
            project.0,
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
        project: ProjectId,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}",
            &URLS.ogc_api.base, &URLS.ogc_api.project, project.0, &URLS.ogc_api.collections
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_collection(&self, client: &HttpClient, collection_id: &str) -> Response {
        let req = client.get(format!(
            "{}{}/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection_id
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_collection(
        &self,
        client: &HttpClient,
        project: ProjectId,
        collection_id: ProjectCollectionId,
    ) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}/{}",
            URLS.ogc_api.base,
            &URLS.ogc_api.project,
            project.0,
            URLS.ogc_api.collections,
            collection_id
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_features(&self, client: &HttpClient, collection_id: &str) -> Response {
        let req = self.get_features_req(client, collection_id);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_features_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection_id: &str,
        params: &T,
    ) -> Response {
        let req = self.get_features_req(client, collection_id).query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_features_req(&self, client: &HttpClient, collection_id: &str) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}/items",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection_id
        ))
    }

    pub async fn get_project_features(
        &self,
        client: &HttpClient,
        collection_id: ProjectCollectionId,
        project: ProjectId,
    ) -> Response {
        let req = self.get_project_features_req(client, collection_id, project);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_project_features_req(
        &self,
        client: &HttpClient,
        collection_id: ProjectCollectionId,
        project: ProjectId,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}{}/{}/items",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project.0,
            URLS.ogc_api.collections,
            collection_id
        ))
    }

    pub async fn get_project_features_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        collection_id: ProjectCollectionId,
        project: ProjectId,
        params: &T,
    ) -> Response {
        let req = self
            .get_project_features_req(client, collection_id, project)
            .query(params);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_feature(&self, client: &HttpClient, collection: &str, id: i32) -> Response {
        let req = self.get_feature_req(client, collection, id);
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_feature_req(&self, client: &HttpClient, collection: &str, id: i32) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}/items/{}",
            &URLS.ogc_api.base, &URLS.ogc_api.collections, collection, id
        ))
    }

    pub async fn get_openapi(&self, client: &HttpClient) -> Response {
        let req = client.get(format!("{}{}", URLS.ogc_api.base, URLS.ogc_api.openapi));
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_openapi(&self, client: &HttpClient, project: ProjectId) -> Response {
        let req = client.get(format!(
            "{}{}/{}{}",
            URLS.ogc_api.base, URLS.ogc_api.project, project.0, URLS.ogc_api.openapi
        ));
        req.send().await.expect(REQUEST_FAILED)
    }

    fn get_project_feature_req(
        &self,
        client: &HttpClient,
        project: ProjectId,
        collection_id: ProjectCollectionId,
        id: i32,
    ) -> RequestBuilder {
        client.get(format!(
            "{}{}/{}{}/{}/items/{}",
            URLS.ogc_api.base,
            URLS.ogc_api.project,
            project.0,
            URLS.ogc_api.collections,
            collection_id,
            id
        ))
    }

    pub async fn get_project_feature_with_params<T: Serialize>(
        &self,
        client: &HttpClient,
        project: ProjectId,
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
        project: ProjectId,
        collection_id: ProjectCollectionId,
        id: i32,
    ) -> Response {
        let req = self.get_project_feature_req(client, project, collection_id, id);
        req.send().await.expect(REQUEST_FAILED)
    }

    pub async fn get_project_collection_ogc(
        &self,
        client: &HttpClient,
        project_id: ProjectId,
        collection_id: ProjectCollectionId,
    ) -> ogcapi_types::common::Collection {
        let response = self
            .get_project_collection(client, project_id, collection_id)
            .await;
        handle_json_response(response)
            .await
            .expect("failed to deserialise")
    }
}
