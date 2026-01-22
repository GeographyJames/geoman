/// A single feature
use crate::{
    URLS,
    constants::GIS_DATA_SCHEMA,
    handlers::{
        ApiError,
        ogc_api::features::{
            Query,
            common::{append_crs_header, retrieve_feature_from_database},
        },
    },
    helpers::get_base_url,
    postgres::PostgresRepo,
    repo::{features, project_features},
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};
use domain::{Feature, FeatureId, GisDataTable, IntoOGCFeature, ProjectId, enums::CollectionId};

#[utoipa::path(
    path = "/collections/{collectionId}/items/{featureId}",
    tag = "OGC API",
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
    let (collection_id, feature_id) = path.into_inner();
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, collection_id
    );

    let Query { crs, .. } = query.into_inner();
    let mut response = match collection_id {
        CollectionId::Projects => {
            let params = project_features::SelectOneParams {
                project_id: ProjectId(feature_id),
                crs: &crs,
            };

            let feature = retrieve_feature_from_database(
                &repo,
                collection_id,
                feature_id,
                collection_url,
                &params,
            )
            .await?;
            HttpResponse::Ok().json(feature)
        }

        CollectionId::DatabaseTable(table) => {
            // Check table exists before querying
            let _table_row: GisDataTable = repo
                .select_one(table.clone())
                .await?
                .ok_or(ApiError::CollectionNotFound)?;
            let params = features::SelectOneParams {
                schema: GIS_DATA_SCHEMA,
                table,
                crs: crs.clone(),
            };
            let feature_id = FeatureId(feature_id);
            let feature: Feature = repo
                .select_one_with_params(feature_id, &params)
                .await?
                .ok_or_else(|| ApiError::FeatureNotFound(feature_id))?;
            let ogc_feature = feature.into_ogc_feature(collection_url);
            HttpResponse::Ok().json(ogc_feature)
        }
        _ => return Err(ApiError::CollectionNotFound),
    };

    append_crs_header(&mut response, &crs);

    Ok(response)
}
