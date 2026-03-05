use actix_web::{
    post,
    web::{self, Json},
};
use domain::enums::LayerCategory;
use domain::{DataProviderLayerId, DataProviderServiceId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderLayerInputPayload {
    pub service_id: DataProviderServiceId,
    pub name: String,
    pub abbreviation: Option<String>,
    pub source: LayerSource,
    pub category: Option<LayerCategory>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub style_config: Option<Value>,
    pub display_options: Option<Value>,
    pub country_code: Option<String>,
    pub subdivision: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum LayerSource {
    MVT { url: String },
    ArcGISRest { service_name: String, layer_id: i64 },
}

impl Default for LayerSource {
    fn default() -> Self {
        Self::MVT { url: String::new() }
    }
}

#[post("")]
#[tracing::instrument(skip(repo, user, payload))]
pub async fn post_data_provider_layer(
    repo: web::Data<PostgresRepo>,
    payload: web::Json<DataProviderLayerInputPayload>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<Json<DataProviderLayerId>, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let id = repo
        .insert(&(payload.into_inner(), user.into_inner().id))
        .await?;
    Ok(Json(id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MockUserCredentials, testing::test_helpers::mock_app};
    use actix_web::test;

    #[actix_web::test]
    async fn post_data_provider_layer_requires_admin_permission() {
        let req = test::TestRequest::post().set_json(&DataProviderLayerInputPayload::default());
        let resp = mock_app(
            post_data_provider_layer,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
