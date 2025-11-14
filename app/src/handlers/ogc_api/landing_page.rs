use crate::{AppState, URLS};
use actix_web::{HttpRequest, get, web};
use ogc::types::common::{
    LandingPage, Link, Linked, MediaType,
    link_relations::{CONFORMANCE, DATA, ROOT, SELF, SERVICE_DESC},
};

#[utoipa::path(
    path = "/",
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
pub async fn get_landing_page(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> web::Json<LandingPage> {
    // Build base URL from request
    let connection_info = req.connection_info();
    let base_url = format!("{}://{}", connection_info.scheme(), connection_info.host(),);
    let ogc_api_base_url = format!("{}{}", base_url, URLS.ogc_api.base);
    let links = [
        Link::new(&ogc_api_base_url, SELF).mediatype(MediaType::Json),
        Link::new(&ogc_api_base_url, ROOT).mediatype(MediaType::Json),
        Link::new(
            format!(
                "{}{}",
                ogc_api_base_url, &URLS.ogc_api.conformance_declaration
            ),
            CONFORMANCE,
        )
        .mediatype(MediaType::Json)
        .title("Conformance declaration"),
        Link::new(format!("{}/collections", ogc_api_base_url), DATA).mediatype(MediaType::Json),
        Link::new(
            format!("{}{}{}", base_url, URLS.docs.base, URLS.docs.api),
            SERVICE_DESC,
        )
        .mediatype(MediaType::OpenApi)
        .title("API definition"),
    ];

    let mut landing_page = state.landing_page.to_owned();
    landing_page.links.insert_or_update(&links);

    web::Json(landing_page)
}
