use chrono::{DateTime, Utc};
use ogcapi_types::common::Crs;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use utoipa::{IntoParams, ToSchema};

#[serde_with::serde_as]
#[derive(Deserialize, Serialize, IntoParams, ToSchema, Default)]
#[into_params(parameter_in = Query)]
#[non_exhaustive]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Query {
    /// The optional limit parameter limits the number of items that are
    /// presented in the response document.
    ///
    /// Only items are counted that are on the first level of the  collection
    /// in the response document. Nested objects contained
    /// within the explicitly requested items shall not be counted.
    #[param(style = Form, required = false, maximum = 10000)]
    pub limit: Option<usize>,

    /// Only features that have a geometry that intersects the bounding box are selected

    #[param(style = Form, explode = false, value_type = Option<Vec<f64>>, required = false, min_items = 4, max_items = 6)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub bbox: Option<ogcapi_types::common::Bbox>,

    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schema(value_type = String)]
    pub bbox_crs: Option<Crs>,

    #[serde(default)]
    #[serde_as(as = "DisplayFromStr")]
    #[param(value_type = String)]
    pub crs: Crs,

    // datetime required by OGC API spec: Requirement 25 /req/core/fc-time-definition
    /// Either a date-time or an interval. Date and time expressions adhere to
    /// RFC 3339. Intervals may be bounded or half-bounded (double-dots at start or end).
    ///
    /// Examples:
    ///
    /// * A date-time: "2018-02-12T23:20:50Z"
    /// * A bounded interval: "2018-02-12T00:00:00Z/2018-03-18T12:31:12Z"
    /// * Half-bounded intervals: "2018-02-12T00:00:00Z/.." or "../2018-03-18T12:31:12Z"
    ///
    /// Only features that have a temporal property that intersects the value
    /// of `datetime` are selected.
    ///
    /// If a feature has multiple temporal properties, it is the decision of
    /// the server whether only a single temporal property is used to determine
    /// the extent or all relevant temporal properties.
    #[param(style = Form, value_type = Option<String>, required = false)]
    pub datetime: Option<DateTime<Utc>>,
}
