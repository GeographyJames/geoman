//! Link Relations
//!
//! [IANA Link Relations Registry](https://www.iana.org/assignments/link-relations/link-relations.xhtml)
//! [OGC Link Relation Type Register](http://www.opengis.net/def/rel)

/// Refers to a resource that identifies the specifications that the link’s context conforms to.
///
/// See: <http://www.opengis.net/def/rel/ogc/1.0/conformance>
pub const CONFORMANCE: &str = "conformance";

/// Conveys an identifier for the link’s context.
pub const SELF: &str = "self";

/// Identifies service description for the context that is primarily intended for consumption by machines.
pub const SERVICE_DESC: &str = "service-desc";

pub const DATA: &str = "data";

pub const ROOT: &str = "root";
