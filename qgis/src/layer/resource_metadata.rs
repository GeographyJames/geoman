use serde::Serialize;

use crate::{layer::ResourceType, srs::Srs};

#[derive(Serialize, Default, Clone)]
pub struct ResourceMetadata {
    identifier: String,
    parentidentifier: String,
    language: String,
    r#type: ResourceType,
    title: String,
    r#abstract: String,
    links: Links,
    dates: Dates,
    fees: String,
    encoding: String,
    crs: Option<Srs>,
    extent: String,
}

#[derive(Serialize, Default, Clone)]
pub struct Links {}

#[derive(Serialize, Default, Clone)]
pub struct Dates {}
