use crate::{
    AppState, URLS, enums::ProjectIdentifier, errors::ApiError, helpers::get_base_url,
    postgres::PostgresRepo,
};
use actix_web::{HttpRequest, get, web};
use domain::Project;
use ogc::{
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
    let base_url = get_base_url(&req);
    let api_url = format!("{}{}", base_url, URLS.ogc_api.base);

    landing_page(&state, &api_url, &base_url)
}

#[get("")]
#[tracing::instrument(skip(repo, project, state, req))]
pub async fn get_project_landing_page(
    repo: web::Data<PostgresRepo>,
    project: web::Path<ProjectIdentifier>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<web::Json<LandingPage>, ApiError> {
    let _project = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or(ApiError::ProjectNotFound(project.clone()))?;
    let base_url = get_base_url(&req);
    let api_url = format!(
        "{}{}{}/{}",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, &project
    );
    let landing_page = landing_page(&state, &api_url, &base_url);
    Ok(landing_page)
}

fn landing_page(app_state: &AppState, api_url: &str, base_url: &str) -> web::Json<LandingPage> {
    let links = [
        Link::new(&api_url, SELF).mediatype(MediaType::Json),
        Link::new(&api_url, ROOT).mediatype(MediaType::Json),
        Link::new(
            format!("{}{}", api_url, &URLS.ogc_api.conformance_declaration),
            CONFORMANCE,
        )
        .mediatype(MediaType::Json)
        .title("Conformance declaration"),
        Link::new(format!("{}/collections", api_url), DATA).mediatype(MediaType::Json),
        Link::new(
            format!("{}{}{}", base_url, URLS.docs.base, URLS.docs.api),
            SERVICE_DESC,
        )
        .mediatype(MediaType::OpenApi)
        .title("API definition"),
    ];

    let mut landing_page = app_state.landing_page.to_owned();
    landing_page.links.insert_or_update(&links);
    web::Json(landing_page)
}
