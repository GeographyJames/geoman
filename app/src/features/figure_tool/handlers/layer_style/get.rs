use actix_web::{HttpResponse, web};

use crate::{
    app::{
        features::figure_tool::dtos::LayerStyleOutputDTO,
        handlers::api::ApiError,
    },
    postgres::PostgresRepo,
};

#[tracing::instrument(skip(repo))]
pub async fn get_layer_styles(
    repo: web::Data<PostgresRepo>,
) -> Result<HttpResponse, actix_web::Error> {
    let res: Vec<LayerStyleOutputDTO> =
        repo.select_all().await.map_err(|e| ApiError::Repository {
            source: e,
            message: "failed to retrieve layer styles from database".into(),
        })?;
    Ok(HttpResponse::Ok().json(res))
}
