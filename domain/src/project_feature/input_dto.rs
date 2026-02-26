use crate::name::NameInputDTO;

pub struct FeatureInputDTO {
    pub name: NameInputDTO,
    pub primary: Option<bool>,
    pub geom_wkb: Vec<u8>,
    pub srid: i32,
    pub target_srid: i32,
}
