use serde::{Deserialize, Serialize};

use crate::SubdivisionId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Subdivision {
    pub id: SubdivisionId,
    pub country_code: isocountry::CountryCode,
    pub subdivision_code: String,
    pub name: String,
}
