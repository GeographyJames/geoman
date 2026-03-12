use actix_web::{HttpResponse, web};

use crate::{
    app::{
        features::figure_tool::dtos::base_map::BaseMapOutputDTO,
        handlers::api::ApiError,
    },
    postgres::PostgresRepo,
};

#[tracing::instrument(skip(repo))]
pub async fn get_base_maps(
    repo: web::Data<PostgresRepo>,
) -> Result<HttpResponse, actix_web::Error> {
    let base_maps: Vec<BaseMapOutputDTO> =
        repo.select_all().await.map_err(|e| ApiError::Repository {
            source: e,
            message: "failed to retrieve base maps from database".into(),
        })?;
    Ok(HttpResponse::Ok().json(base_maps))
}
