use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::ogc::types::common::Bbox;

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields)]
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
    pub bbox: Option<Bbox>,

    /// Either a date-time or an interval, open or closed. Date and time expressions adhere to RFC 3339
    #[param(style = Form, value_type = Option<String>, required = false)]
    pub datetime: Option<DateTime<Utc>>,
}
