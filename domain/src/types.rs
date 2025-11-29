use ogcapi_types::common::Crs;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SupportedCrs(Vec<Crs>);

impl SupportedCrs {
    pub fn new(storage_crs: Option<Crs>) -> Self {
        let default_crs = Crs::default();
        let crs = if let Some(ref crs) = storage_crs
            && default_crs.as_srid() != crs.as_srid()
        {
            vec![default_crs, crs.clone()]
        } else {
            vec![default_crs]
        };
        Self(crs)
    }
    pub fn into_inner(self) -> Vec<Crs> {
        self.0
    }
}
