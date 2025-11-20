use ogcapi_types::common::Crs;

/// Valid CRS that can only be obtained by checking against the list of CRS accepted by the application
#[derive(Clone, Default)]
pub struct ValidCrs(Crs);

impl ValidCrs {
    pub fn new(valid: &Vec<Crs>, crs: Crs) -> Result<Self, Crs> {
        if valid
            .iter()
            .any(|supported| supported.as_srid() == crs.as_srid())
        {
            Ok(ValidCrs(crs))
        } else {
            Err(crs)
        }
    }
}

impl AsRef<Crs> for ValidCrs {
    fn as_ref(&self) -> &Crs {
        &self.0
    }
}
