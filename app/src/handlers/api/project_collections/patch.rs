use actix_web::{patch, web, HttpResponse};
use domain::{CollectionUpdateDto, ProjectCollectionId};
use serde::Deserialize;

use crate::{errors::ApiError, postgres::PostgresRepo, types::AuthenticatedUser};

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

#[derive(Deserialize)]
pub struct PatchCollectionPayload {
    pub title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub description: Option<Option<String>>,
}

#[patch("/{id}")]
#[tracing::instrument(skip(repo, body, user))]
pub async fn patch_collection(
    id: web::Path<ProjectCollectionId>,
    body: web::Json<PatchCollectionPayload>,
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    let payload = body.into_inner();
    let dto = CollectionUpdateDto {
        id: id.into_inner(),
        title: payload.title,
        description: payload.description,
    };
    repo.update(&(&dto, user.id)).await?;
    Ok(HttpResponse::NoContent().finish())
}
