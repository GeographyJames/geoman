pub struct FeatureInputDTO {
    pub name: String,
    pub primary: bool,
    pub geom: gdal::vector::Geometry,
}
