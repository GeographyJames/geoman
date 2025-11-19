#[derive(Clone)]
pub struct SupportedCrs {
    pub srid: i32,
    pub auth_name: String,
}

pub struct AllSupportedCrs(Vec<SupportedCrs>);

impl From<Vec<SupportedCrs>> for AllSupportedCrs {
    fn from(value: Vec<SupportedCrs>) -> Self {
        Self(value)
    }
}

impl TryInto<Vec<ogcapi_types::common::Crs>> for AllSupportedCrs {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<ogcapi_types::common::Crs>, Self::Error> {
        self.0.into_iter().map(|crs| crs.try_into()).collect()
    }
}

impl TryInto<ogcapi_types::common::Crs> for SupportedCrs {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<ogcapi_types::common::Crs, Self::Error> {
        if self.auth_name.to_lowercase() != "epsg" {
            return Err(anyhow::anyhow!(
                "Failed to convert SupportedCrs to ogcapi Crs: Only EPSG is supported. SupportedCrs auth name: {}",
                self.auth_name
            ));
        }
        Ok(ogcapi_types::common::Crs::from_srid(self.srid))
    }
}
