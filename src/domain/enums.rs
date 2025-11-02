use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Country {
    Scotland,
    England,
    Wales,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Active,
    Archived,
    Deleted,
}
