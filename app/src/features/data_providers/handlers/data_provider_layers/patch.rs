use actix_web::{HttpResponse, patch, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    AuthenticatedUser,
    errors::ApiError,
    features::data_providers::types::{DataProviderLayerId, LayerCategory},
    postgres::PostgresRepo,
};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderLayerUpdatePayload {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub abbreviation: Option<Option<String>>,
    pub source: Option<Value>,
    pub category: Option<LayerCategory>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub description: Option<Option<String>>,
    pub enabled_geoman: Option<bool>,
    pub style_config: Option<Value>,
    pub display_options: Option<Value>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub country_code: Option<Option<String>>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub subdivision: Option<Option<String>>,
    pub sort_order: Option<i32>,
    pub enabled_figure_tool: Option<bool>,
}

#[patch("/{id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_data_provider_layer(
    repo: web::Data<PostgresRepo>,
    body: web::Json<DataProviderLayerUpdatePayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<DataProviderLayerId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let user = user.into_inner();
    repo.update(&(body.into_inner(), id.into_inner(), user.id))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AuthenticatedUser, MockUserCredentials, testing::test_helpers::mock_app_with_path_params,
    };
    use actix_web::test;

    #[actix_web::test]
    async fn patch_data_provider_layer_requires_admin_permission() {
        let req = test::TestRequest::patch()
            .uri("/1")
            .set_json(&DataProviderLayerUpdatePayload::default());
        let resp = mock_app_with_path_params(
            patch_data_provider_layer,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
