use actix_web::{HttpResponse, patch, web};
use domain::enums::Status;
use serde::{Deserialize, Serialize};

use crate::{
    AuthenticatedUser,
    errors::ApiError,
    features::{
        data_providers::DataProviderLayerId,
        figure_tool::{
            dtos::FigureProperties, handlers::figure::FigureLayerPayload, ids::FigureId,
        },
    },
    postgres::PostgresRepo,
};

#[derive(Deserialize, Serialize, Default)]
pub struct FigureUpdatePayload {
    pub scale: Option<u32>,
    pub legend_width_mm: Option<u32>,
    pub margin_mm: Option<u32>,
    pub page_width_mm: Option<u32>,
    pub page_height_mm: Option<u32>,
    pub srid: Option<u16>,
    pub status: Option<Status>,
    pub properties: Option<FigureProperties>,
    pub layers: Option<Vec<FigureLayerPayload>>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub main_map_base_map_id: Option<Option<DataProviderLayerId>>,
    #[serde(default, deserialize_with = "crate::serde_helpers::double_option")]
    pub overview_map_base_map_id: Option<Option<DataProviderLayerId>>,
}

#[patch("/{figure_id}")]
#[tracing::instrument(skip(repo, body, user, figure_id))]
pub async fn patch_figure(
    repo: web::Data<PostgresRepo>,
    body: web::Json<FigureUpdatePayload>,
    user: web::ReqData<AuthenticatedUser>,
    figure_id: web::Path<FigureId>,
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    repo.update(&(body.into_inner(), figure_id.into_inner(), user.id))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
