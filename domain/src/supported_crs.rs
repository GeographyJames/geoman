use ogcapi_types::common::Crs;

pub struct SupportedCrs {
    pub srid: i32,
    pub auth_name: String,
}

impl TryInto<Crs> for SupportedCrs {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<Crs, Self::Error> {
        if self.auth_name.to_lowercase() != "epsg" {
            return Err(anyhow::anyhow!(
                "Failed to convert SupportedCrs to ogcapi Crs: Only EPSG is supported. SupportedCrs auth name: {}",
                self.auth_name
            ));
        }
        Ok(Crs::from_srid(self.srid))
    }
}
