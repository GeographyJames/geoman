use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use domain::{FeatureId, ProjectId, TurbineLayout};
use geo::shapefile_builder::{self, TurbineEntry};
use ogcapi_types::common::Crs;
use serde::Deserialize;
use strum::Display;

use crate::{
    AuthenticatedUser, constants::TURBINE_LAYOUTS_COLLECTION_ID, errors::ApiError,
    postgres::PostgresRepo, repo::project_features::SelectOneParams,
};

#[derive(Deserialize, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum FeatureFormat {
    Shapefile,
    Csv,
    Geojson,
}

impl FeatureFormat {
    fn file_extension(&self) -> String {
        match self {
            FeatureFormat::Shapefile => "shz".to_string(),
            FeatureFormat::Csv => "csv".to_string(),
            FeatureFormat::Geojson => "geojson".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct FeatureDownloadPath {
    project_slug: String,
    collection_slug: String,
    feature_id: FeatureId,
}

#[derive(Deserialize)]
pub struct FeatureDownloadQuery {
    format: FeatureFormat,
}

#[tracing::instrument(skip(repo, path, query))]
#[get("/{project_slug}/{collection_slug}/{feature_id}")]
pub async fn get_project_feature_download(
    repo: web::Data<PostgresRepo>,
    path: web::Path<FeatureDownloadPath>,
    query: web::Query<FeatureDownloadQuery>,
    _user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    let FeatureDownloadPath {
        project_slug,
        collection_slug,
        feature_id,
    } = path.into_inner();
    match query.into_inner().format {
        FeatureFormat::Shapefile => {
            get_shapefile(&repo, feature_id, &project_slug, &collection_slug).await
        }
        FeatureFormat::Csv => get_csv(&repo, feature_id, &project_slug, &collection_slug).await,
        FeatureFormat::Geojson => {
            get_geojson(&repo, feature_id, &project_slug, &collection_slug).await
        }
    }
}

async fn get_csv(
    repo: &PostgresRepo,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<HttpResponse, ApiError> {
    let rows = repo
        .get_turbine_layout_csv(feature_id, project_slug, collection_slug)
        .await
        .context("failed to query turbine layout csv")?;

    if rows.is_empty() {
        return Err(ApiError::FeatureNotFound(feature_id));
    }

    let srid = rows[0].srid;
    let layout_name = &rows[0].layout_name;
    let mut csv =
        format!("id,turbine_number,hub_height_m,rotor_diameter_m,x_epsg:{srid},y_epsg:{srid}\n");
    let download_filename = download_filename(
        project_slug,
        collection_slug,
        feature_id,
        layout_name,
        FeatureFormat::Csv,
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            row.id,
            row.turbine_number,
            row.hub_height_m.map_or(String::new(), |v| v.to_string()),
            row.rotor_diameter_m
                .map_or(String::new(), |v| v.to_string()),
            row.x,
            row.y,
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", download_filename),
        ))
        .body(csv))
}

async fn get_shapefile(
    repo: &PostgresRepo,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<HttpResponse, ApiError> {
    let collection_id = repo
        .get_collection_id_by_slug(collection_slug)
        .await
        .context("failed to query collection id")?
        .ok_or(ApiError::CollectionNotFound)?;

    if collection_id == TURBINE_LAYOUTS_COLLECTION_ID {
        get_turbine_layout_shapefile(repo, feature_id, project_slug, collection_slug).await
    } else {
        get_project_feature_shapefile(repo, feature_id, project_slug, collection_slug).await
    }
}

async fn get_project_feature_shapefile(
    repo: &PostgresRepo,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<HttpResponse, ApiError> {
    let ft = repo
        .get_project_feature_for_download(feature_id, project_slug, collection_slug)
        .await
        .context("failed to query project feature")?
        .ok_or(ApiError::FeatureNotFound(feature_id))?;

    let layer_name = layer_name(collection_slug, feature_id, &ft.name);
    let bytes = shapefile_builder::build_project_feature_shapefile(
        ft.srid as u32,
        ft.geom_type,
        &layer_name,
        &ft.geom,
    )?;

    Ok(shapefile_response(
        bytes,
        &download_filename(
            project_slug,
            collection_slug,
            feature_id,
            &ft.name,
            FeatureFormat::Shapefile,
        ),
    ))
}

async fn get_turbine_layout_shapefile(
    repo: &PostgresRepo,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<HttpResponse, ApiError> {
    let rows = repo
        .get_turbine_layout_shapefile(feature_id, project_slug, collection_slug)
        .await
        .context("failed to query turbine layout shapefile")?;

    if rows.is_empty() {
        return Err(ApiError::FeatureNotFound(feature_id));
    }

    let layout_name = rows[0].layout_name.clone();
    let srid = rows[0].srid;
    let layer_name = layer_name(collection_slug, feature_id, &layout_name);
    let turbines: Vec<TurbineEntry> = rows
        .into_iter()
        .map(|r| TurbineEntry {
            id: r.id,
            turbine_number: r.turbine_number,
            hub_height_m: r.hub_height_m,
            rotor_diameter_m: r.rotor_diameter_m,
            geom_wkb: r.geom,
        })
        .collect();
    let bytes =
        shapefile_builder::build_turbine_layout_shapefile(srid as u32, &layer_name, &turbines)?;

    Ok(shapefile_response(
        bytes,
        &download_filename(
            project_slug,
            collection_slug,
            feature_id,
            &layout_name,
            FeatureFormat::Shapefile,
        ),
    ))
}

async fn get_geojson(
    repo: &PostgresRepo,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<HttpResponse, ApiError> {
    let collection_id = repo
        .get_collection_id_by_slug(collection_slug)
        .await
        .context("failed to query collection id")?
        .ok_or(ApiError::CollectionNotFound)?;

    if collection_id != TURBINE_LAYOUTS_COLLECTION_ID {
        return Err(ApiError::CollectionNotFound);
    }

    let project_id = repo
        .get_project_id_by_slug(project_slug)
        .await
        .context("failed to query project id")?
        .ok_or(ApiError::NotFound)?;

    let crs = Crs::from_srid(4326);
    let params = SelectOneParams {
        project_id: ProjectId(project_id),
        crs: &crs,
    };

    let layout = repo
        .select_one_with_params::<TurbineLayout, _>(feature_id, &params)
        .await?
        .ok_or(ApiError::FeatureNotFound(feature_id))?;

    let storage_crs_srid = layout.properties.storage_crs_srid;
    let storage_crs_name = layout.properties.storage_crs_name.clone();

    let features: Vec<geojson::Feature> = layout
        .turbines
        .into_iter()
        .map(|t| {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "turbine_number".to_string(),
                serde_json::json!(t.turbine_number),
            );
            properties.insert(
                "hub_height_mm".to_string(),
                serde_json::json!(t.hub_height_mm),
            );
            properties.insert(
                "rotor_diameter_mm".to_string(),
                serde_json::json!(t.rotor_diameter_mm),
            );
            properties.insert("x_storage_crs".to_string(), serde_json::json!(t.x_storage_crs));
            properties.insert("y_storage_crs".to_string(), serde_json::json!(t.y_storage_crs));
            geojson::Feature {
                id: Some(geojson::feature::Id::Number(serde_json::Number::from(t.id))),
                geometry: Some(t.geometry),
                properties: Some(properties),
                bbox: None,
                foreign_members: None,
            }
        })
        .collect();

    let mut foreign_members = serde_json::Map::new();
    foreign_members.insert(
        "storage_crs_srid".to_string(),
        serde_json::json!(storage_crs_srid),
    );
    foreign_members.insert(
        "storage_crs_name".to_string(),
        serde_json::json!(storage_crs_name),
    );

    let feature_collection = geojson::FeatureCollection {
        bbox: None,
        features,
        foreign_members: Some(foreign_members),
    };

    Ok(HttpResponse::Ok()
        .content_type("application/geo+json")
        .json(feature_collection))
}

fn layer_name(collection_slug: &str, feature_id: FeatureId, name: &str) -> String {
    format!(
        "{}{:05}-{}",
        collection_slug,
        feature_id.0,
        slug::slugify(name)
    )
}

fn download_filename(
    project_slug: &str,
    collection_slug: &str,
    feature_id: FeatureId,
    feature_name: &str,
    format: FeatureFormat,
) -> String {
    format!(
        "{}-{}{:05}-{}.{}",
        project_slug,
        collection_slug,
        feature_id.0,
        slug::slugify(feature_name),
        format.file_extension()
    )
}

fn shapefile_response(content: Vec<u8>, filename: &str) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(content)
}
