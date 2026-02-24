use serde::Deserialize;

use crate::{
    app::handlers::api::ApiError,
    domain::dtos::{NameInputDTO, TeamInputDTO},
};

#[derive(Deserialize)]
pub struct TeamInputPayload {
    pub name: String,
}

impl TryFrom<TeamInputPayload> for TeamInputDTO {
    type Error = ApiError;

    fn try_from(payload: TeamInputPayload) -> Result<Self, Self::Error> {
        let TeamInputPayload { name } = payload;
        Ok(TeamInputDTO {
            name: NameInputDTO::parse(name)
                .map_err(|e| ApiError::Validation(format!("Invalid name: {}", e)))?,
        })
    }
}
