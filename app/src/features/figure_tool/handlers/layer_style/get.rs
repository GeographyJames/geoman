use actix_web::{
    get,
    web::{self, Json},
};

use crate::{
    errors::ApiError, features::figure_tool::dtos::LayerStyleOutputDTO, postgres::PostgresRepo,
};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_layer_styles(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<LayerStyleOutputDTO>>, ApiError> {
    let styles: Vec<LayerStyleOutputDTO> = repo.select_all().await?;
    Ok(Json(styles))
}
