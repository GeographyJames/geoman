use actix_web::{HttpRequest, HttpResponse, get, web};
use domain::enums::CollectionId;
use ogc::features::filtering::{QueryableProperty, Queryables};
use std::collections::HashMap;

use crate::{URLS, errors::ApiError, helpers::get_base_url};

#[get("/{collectionId}/queryables")]
#[tracing::instrument(skip(req, collection_id))]
pub async fn get_collection_queryables(
    req: HttpRequest,
    collection_id: web::Path<CollectionId>,
) -> Result<HttpResponse, ApiError> {
    let base_url = get_base_url(&req);
    let queryables_url = format!(
        "{}{}{}/{}/queryables",
        base_url,
        URLS.ogc_api.base,
        URLS.ogc_api.collections,
        collection_id.as_ref()
    );

    match collection_id.into_inner() {
        CollectionId::Projects => {
            let mut properties = HashMap::new();

            // Add status queryable property
            properties.insert(
                "status".to_string(),
                QueryableProperty {
                    title: Some("Project Status".to_string()),
                    description: Some("The status of the project".to_string()),
                    r#type: Some("string".to_string()),
                    r#enum: Some(vec![
                        "ACTIVE".to_string(),
                        "ARCHIVED".to_string(),
                        "DELETED".to_string(),
                    ]),
                },
            );

            let queryables = Queryables {
                schema: "https://json-schema.org/draft/2020-12/schema".to_string(),
                id: queryables_url,
                r#type: "object".to_string(),
                title: Some("Projects".to_string()),
                description: Some("Queryable properties for the Projects collection".to_string()),
                properties,
                additional_properties: false,
            };

            Ok(HttpResponse::Ok()
                .content_type("application/schema+json")
                .json(queryables))
        }
        _ => Err(ApiError::NotFound),
    }
}
