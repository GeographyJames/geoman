use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ogc::types::common::media_types::MediaType;

/// Hyperlink to enable Hypermedia Access
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Eq, Clone)]
pub struct Link {
    /// Supplies the URI to a remote resource (or resource fragment).
    pub href: String,
    /// The type or semantics of the relation.
    pub rel: String,
    /// A hint indicating what the media type of the result of dereferencing
    /// the link should be.
    #[schema(nullable = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MediaType>,
    /// Used to label the destination of a link such that it can be used as a
    /// human-readable identifier.
    #[schema(nullable = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Link {
    /// Constructs a new Link with the given href and link relation
    pub fn new(href: impl ToString, rel: impl ToString) -> Link {
        Link {
            href: href.to_string(),
            rel: rel.to_string(),
            r#type: None,
            title: None,
        }
    }

    /// Sets the media type of the Link and returns the Value
    pub fn mediatype(mut self, media_type: MediaType) -> Link {
        self.r#type = Some(media_type);
        self
    }

    /// Sets the title of the Link and returns the Value
    pub fn title(mut self, title: impl ToString) -> Link {
        self.title = Some(title.to_string());
        self
    }
}
