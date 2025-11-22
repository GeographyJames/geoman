/// A single feature
use crate::{
    URLS,
    handlers::{
        ApiError,
        ogc_api::features::{
            Query,
            common::{append_crs_header, retrieve_feature_from_database},
        },
    },
    helpers::get_base_url,
    postgres::PostgresRepo,
    repo::project_features::SelectOneParams,
    types::ValidCrs,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};
use domain::enums::CollectionId;
use ogcapi_types::common::Crs;

#[utoipa::path(
    path = "/collections/{collectionId}/items/{featureId}",
    params(
        ("collectionId" = String, Path, description = "Identifier of a collection"),
        ("featureId" = i32, Path, description = "Identifier of a feature"),
    ),
    responses(
        (status = 200, description = "A single feature from the collection"),
        (status = 404, description = "Collection or feature not found")
    )
)]
#[get("/{collectionId}/items/{featureId}")]
#[tracing::instrument(skip(repo, req, path, query))]
pub async fn get_feature(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(CollectionId, i32)>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let valid_crs: Vec<Crs> = repo.select_all().await?;
    let (collection, feature_id) = path.into_inner();
    let Query { crs, .. } = query.into_inner();
    let request_crs = ValidCrs::new(&valid_crs, crs).map_err(ApiError::UnsupportedRequestCrs)?;
    let params = SelectOneParams {
        project_id: None,
        crs: &request_crs,
    };
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, collection
    );
    let feature =
        retrieve_feature_from_database(&repo, collection, feature_id, collection_url, &params)
            .await?;
    let mut response = HttpResponse::Ok().json(feature);
    append_crs_header(&mut response, &request_crs);

    Ok(response)
}
