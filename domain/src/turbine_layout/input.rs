use std::collections::HashSet;

use anyhow::{Context, anyhow};
use gdal::{
    cpl::CslStringList,
    spatial_ref::CoordTransform,
    vector::{Feature, Geometry, OGRwkbGeometryType, geometry_type_to_name},
};
use geojson::Value;

use crate::name::NameInputDTO;

#[derive(Debug)]
pub struct TurbineLayoutInputDTO {
    pub name: NameInputDTO,
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
pub struct TurbinesGeomInputDTO(Vec<TurbineInputDTO>);

impl AsRef<Vec<TurbineInputDTO>> for TurbinesGeomInputDTO {
    fn as_ref(&self) -> &Vec<TurbineInputDTO> {
        &self.0
    }
}

impl TryInto<geojson::Position> for TurbineInputDTO {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<geojson::Position, Self::Error> {
        let point = self.geom.get_point(0);
        Ok(geojson::Position::from([point.0, point.1]))
    }
}

impl TurbinesGeomInputDTO {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl TryInto<geojson::Geometry> for TurbinesGeomInputDTO {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<geojson::Geometry, Self::Error> {
        let points = Value::MultiPoint(
            self.0
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<Vec<geojson::Position>, _>>()?,
        );
        Ok(geojson::Geometry::new(points))
    }
}

#[derive(Debug)]
pub struct LayoutBuilder {
    hub_height_default_mm: Option<u32>,
    blade_length_default_mm: Option<u32>,
    turbine_number_field: Option<String>,
    blade_length_field: Option<String>,
    hub_height_field: Option<String>,
    turbines: TurbinesGeomInputDTO,
    default_turbine_number: u32,
    used_turbine_numbers: HashSet<u32>,
}

impl LayoutBuilder {
    pub fn new(
        hub_height_default_mm: Option<u32>,
        blade_length_default_mm: Option<u32>,
        turbine_number_field: Option<String>,
        blade_length_field: Option<String>,
        hub_height_field: Option<String>,
    ) -> Self {
        Self {
            hub_height_default_mm,
            blade_length_default_mm,
            turbine_number_field,
            blade_length_field,
            hub_height_field,
            turbines: TurbinesGeomInputDTO::new(),
            default_turbine_number: 0,
            used_turbine_numbers: HashSet::new(),
        }
    }

    pub fn turbines(self) -> TurbinesGeomInputDTO {
        self.turbines
    }

    pub fn add_feature(&mut self, ft: Feature) -> Result<(), anyhow::Error> {
        if let Some(geom) = ft.geometry() {
            let geom = geom
                .make_valid(&CslStringList::new())
                .context("failed to make geometry valid!!")?;

            let hh = self.get_hub_height(&ft)?;
            let rd = self.get_rotor_diameter(&ft)?;

            match geom.geometry_type() {
                OGRwkbGeometryType::wkbMultiPoint => {
                    self.add_multipoint(geom, hh, rd)?;
                }
                OGRwkbGeometryType::wkbPoint => {
                    let turbine_number = self.get_turbine_number(&ft)?;
                    self.add_point(geom, hh, rd, turbine_number)?;
                }
                other => {
                    return Err(anyhow!(
                        "Invalid geometry type ({}). Geometry must be Point or MultiPoint",
                        geometry_type_to_name(other)
                    ));
                }
            }
        }
        Ok(())
    }

    fn get_hub_height(&self, ft: &Feature) -> Result<Option<u32>, anyhow::Error> {
        let hub_height_mm = match self.hub_height_field {
            Some(ref field) => {
                let idx = ft
                    .field_index(field)
                    .map_err(|e| anyhow!("failed to get hub height field: {}", e))?;
                match ft
                    .field_as_double(idx)
                    .map_err(|e| anyhow!("failed to get hub height field: {}", e))?
                {
                    None => self.hub_height_default_mm,
                    Some(num) if num > 0. => Some((num * 1000.) as u32),
                    Some(0.) => {
                        return Err(anyhow!(
                            "Field used for hub height returned zero. This may mean the field has an unsuported data type. Check the data is a numeric type and ensure all hub heights are positive and non zero"
                        ));
                    }
                    _ => return Err(anyhow!("hub heights must be positive and non zero")),
                }
            }
            None => self.hub_height_default_mm,
        };
        Ok(hub_height_mm)
    }

    fn get_rotor_diameter(&self, ft: &Feature) -> Result<Option<u32>, anyhow::Error> {
        let rotor_diameter_mm = match self.blade_length_field {
            Some(ref field) => {
                let idx = ft
                    .field_index(field)
                    .map_err(|e| anyhow!("failed to get blade length field: {}", e))?;
                match ft
                    .field_as_double(idx)
                    .map_err(|e| anyhow!("failed to get blade length field: {}", e))?
                {
                    None => self.hub_height_default_mm,
                    Some(0.) => {
                        return Err(anyhow!(
                            "Field used for blade length returned zero. This may mean the field has an unsuported data type. Check the data is a numeric type and ensure all blade lengths are positive and non zero"
                        ));
                    }
                    Some(num) if num > 0. => Some((num * 1000.) as u32),
                    _ => return Err(anyhow!("blade lengths must be positive and non zero")),
                }
            }
            None => self.blade_length_default_mm,
        };
        Ok(rotor_diameter_mm)
    }

    fn get_turbine_number(&mut self, ft: &Feature) -> Result<u32, anyhow::Error> {
        let turbine_number = match self.turbine_number_field {
            Some(ref field) => {
                let idx = ft
                    .field_index(field)
                    .map_err(|e| anyhow!("failed to get turbine number field: {}", e))?;
                match ft
                    .field_as_integer(idx)
                    .map_err(|e| anyhow!("failed to get turbine number field: {}", e))?
                {
                    None => {
                        return Err(anyhow!(
                            "Turbine number field is null for one or more turbines. All turbines must be numbered when selecting a field for turbine numbering"
                        ));
                    }
                    Some(0) => {
                        return Err(anyhow!(
                            "Field used for turbine number returned zero. This may mean the field has an unsuported data type. Check the data type is numeric and ensure all turbine numbers are non zero positive integers"
                        ));
                    }
                    Some(num) if num > 0 => num as u32,
                    _ => {
                        return Err(anyhow!(
                            "turbine numbers must be positive non zero integers"
                        ));
                    }
                }
            }
            None => {
                self.default_turbine_number += 1;
                self.default_turbine_number
            }
        };
        Ok(turbine_number)
    }

    fn add_multipoint(
        &mut self,
        multipoint: Geometry,
        hub_height_mm: Option<u32>,
        rotor_diameter_mm: Option<u32>,
    ) -> Result<(), anyhow::Error> {
        for i in 0..multipoint.geometry_count() {
            let point_ref = multipoint.get_geometry(i);
            let point = point_ref.clone();
            if point.is_valid()
                && !point.is_empty()
                && point.geometry_type() == OGRwkbGeometryType::wkbPoint
            {
                self.default_turbine_number += 1;
                self.add_point(
                    point,
                    hub_height_mm,
                    rotor_diameter_mm,
                    self.default_turbine_number,
                )?;
            }
        }
        Ok(())
    }

    fn add_point(
        &mut self,
        point: Geometry,
        hub_height_mm: Option<u32>,
        rotor_diameter_mm: Option<u32>,
        turbine_number: u32,
    ) -> Result<(), anyhow::Error> {
        self.turbines.0.push(TurbineInputDTO {
            hub_height_mm,
            rotor_diameter_mm,
            turbine_number,
            geom: point,
        });
        if !self.used_turbine_numbers.insert(turbine_number) {
            return Err(anyhow!(
                "Turbine numbering has duplicates. All turbine numbers must be unique."
            ));
        }
        Ok(())
    }
}
