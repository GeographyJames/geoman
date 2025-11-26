use crate::{
    AppState, URLS, constants::OPEN_API_JSON, handlers::ApiError, helpers::get_base_url,
    postgres::PostgresRepo,
};
use actix_web::{HttpRequest, get, web};
use domain::{ProjectId, project::ProjectName};

use ogcapi_types::common::{
    LandingPage, Link, Linked,
    link_rel::{CONFORMANCE, DATA, ROOT, SELF, SERVICE_DESC},
    media_type::JSON,
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
    let base_url = get_base_url(&req);
    let api_url = format!("{}{}", base_url, URLS.ogc_api.base);
    landing_page(&state, &api_url)
}

#[get("")]
#[tracing::instrument(skip(repo, project_id, state, req))]
pub async fn get_project_landing_page(
    repo: web::Data<PostgresRepo>,
    project_id: web::Path<ProjectId>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<web::Json<LandingPage>, ApiError> {
    let _project = repo
        .select_one::<ProjectName>(*project_id)
        .await?
        .ok_or(ApiError::ProjectNotFound(*project_id))?;
    let base_url = get_base_url(&req);
    let api_url = format!(
        "{}{}{}/{}",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project_id
    );
    let landing_page = landing_page(&state, &api_url);
    Ok(landing_page)
}

fn landing_page(app_state: &AppState, api_url: &str) -> web::Json<LandingPage> {
    let links = [
        Link::new(api_url, SELF).mediatype(JSON),
        Link::new(api_url, ROOT).mediatype(JSON),
        Link::new(
            format!("{}{}", api_url, &URLS.ogc_api.conformance_declaration),
            CONFORMANCE,
        )
        .mediatype(JSON)
        .title("Conformance declaration"),
        Link::new(format!("{}/collections", api_url), DATA).mediatype(JSON),
        Link::new(format!("{}{}", api_url, URLS.ogc_api.openapi), SERVICE_DESC)
            .mediatype(OPEN_API_JSON)
            .title("API definition"),
    ];

    let mut landing_page = app_state.landing_page.to_owned();
    landing_page.links.insert_or_update(&links);
    web::Json(landing_page)
}
