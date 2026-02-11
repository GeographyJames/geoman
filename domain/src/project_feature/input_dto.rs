pub struct FeatureInputDTO {
    pub name: String,
    pub primary: Option<bool>,
    pub geom_wkb: Vec<u8>,
    pub srid: i32,
}
