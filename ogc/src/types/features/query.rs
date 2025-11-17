use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use utoipa::IntoParams;

#[serde_with::serde_as]
#[derive(Deserialize, Serialize, IntoParams, Default)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields, default)]
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

    /// Either a date-time or an interval, open or closed. Date and time expressions adhere to RFC 3339
    #[param(style = Form, value_type = Option<String>, required = false)]
    pub datetime: Option<DateTime<Utc>>,
}
