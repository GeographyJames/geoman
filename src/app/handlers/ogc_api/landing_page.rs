use crate::app::{AppState, URLS};
use crate::ogc::types::common::link_relations::{CONFORMANCE, DATA, ROOT, SELF, SERVICE_DESC};
use crate::ogc::types::common::media_types::{JSON, OPEN_API_JSON};
use crate::ogc::types::common::{LandingPage, Link, Linked};
use actix_web::{HttpRequest, HttpResponse, get, web};

#[utoipa::path(
    responses(
        (
            status = 200,
            description = "The landing page provides links to the API \
            definition (link relations `service-desc` and `service-doc`), and \
            the Conformance declaration (path `/conformance`, link relation \
            `conformance`).", 
            body = LandingPage
        ),
    ),
)]
#[get("")]
#[tracing::instrument(skip(req, state))]
pub async fn get_landing_page(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    // Build base URL from request
    let connection_info = req.connection_info();
    let base_url = format!("{}://{}", connection_info.scheme(), connection_info.host(),);
    let ogc_api_base_url = format!("{}{}", base_url, URLS.ogc_api.base);
    let links = [
        Link::new(&ogc_api_base_url, SELF).mediatype(JSON),
        Link::new(&ogc_api_base_url, ROOT).mediatype(JSON),
        Link::new(
            format!(
                "{}{}",
                ogc_api_base_url, &URLS.ogc_api.conformance_declaration
            ),
            CONFORMANCE,
        )
        .mediatype(JSON)
        .title("Conformance declaration"),
        Link::new(format!("{}/collections", ogc_api_base_url), DATA).mediatype(JSON),
        Link::new(
            format!("{}{}{}", base_url, URLS.docs.base, URLS.docs.api),
            SERVICE_DESC,
        )
        .mediatype(OPEN_API_JSON)
        .title("API definition"),
    ];

    let mut landing_page = state.landing_page.to_owned();
    landing_page.links.insert_or_update(&links);

    HttpResponse::Ok().json(landing_page)
}
