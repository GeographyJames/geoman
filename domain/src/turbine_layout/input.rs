use crate::FeatureNameInputDTO;

#[derive(Debug)]
pub struct TurbineLayoutInputDTO {
    pub name: FeatureNameInputDTO,
    pub primary: Option<bool>,
    pub turbines: TurbinesGeomInputDTO,
    pub target_srid: i32,
    pub srid: i32,
}

#[derive(Debug)]
pub struct TurbineInputDTO {
    pub turbine_number: u32,
    pub hub_height_mm: Option<u32>,
    pub rotor_diameter_mm: Option<u32>,
    pub geom: gdal::vector::Geometry,
}

#[derive(Debug, Default)]
pub struct TurbinesGeomInputDTO(pub Vec<TurbineInputDTO>);

impl AsRef<Vec<TurbineInputDTO>> for TurbinesGeomInputDTO {
    fn as_ref(&self) -> &Vec<TurbineInputDTO> {
        &self.0
    }
}

impl TurbinesGeomInputDTO {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

pub struct DuplicateTurbineInputDTO {
    pub name: Option<FeatureNameInputDTO>,
    pub hub_height_mm: Option<i32>,
    pub rotor_diameter_mm: Option<i32>,
    pub primary: Option<bool>,
}
