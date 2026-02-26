use anyhow::{Context, anyhow};
use gdal::{
    spatial_ref::{CoordTransform, SpatialRef},
    vector::{Feature, Layer, LayerAccess},
};

use crate::turbine_layout::{LayoutBuilder, TurbinesGeomInputDTO};

pub struct InputDTOBuilder<'a> {
    _ds: &'a gdal::Dataset,
    layer: Layer<'a>,
    input_srs: SpatialRef,

    valid_feature_ids: Vec<u64>,
    empty_geometries: i32,
    features_with_no_geometry: i32,
}

impl<'a> InputDTOBuilder<'a> {
    pub fn new(ds: &'a gdal::Dataset) -> Result<Self, anyhow::Error> {
        let mut layer = ds.layers().next().context("dataset has no layers")?;
        let input_srs = layer
            .spatial_ref()
            .context("The layer has no spatial reference system")?;
        if layer.feature_count() == 0 {
            return Err(anyhow!("layer has no features"));
        }
        let mut valid_feature_ids = Vec::new();
        let mut features_with_no_geometry = 0;
        let mut empty_geometries = 0;

        for feature in layer.features() {
            let fid = feature.fid().expect("feature has no id");
            match feature.geometry() {
                None => features_with_no_geometry += 1,
                Some(geom) => {
                    if geom.is_empty() {
                        empty_geometries += 1
                    } else {
                        valid_feature_ids.push(fid)
                    }
                }
            }
        }
        let mut builder = Self {
            _ds: ds,
            layer,
            input_srs,

            valid_feature_ids,
            features_with_no_geometry,
            empty_geometries,
        };

        Ok(builder)
    }

    pub fn warnings(&self) -> Option<Vec<String>> {
        let mut warnings = Vec::new();
        if self.empty_geometries > 0 {
            warnings.push(format!(
                "Layer contains {} feature(s) with empty geometry. These will be removed",
                self.empty_geometries
            ))
        }
        if self.features_with_no_geometry > 0 {
            warnings.push(format!(
                "Layer contains {} feature(s) with no geometry. These will be removed",
                self.features_with_no_geometry
            ));
        }
        if warnings.is_empty() {
            return Some(warnings);
        }
        None
    }

    fn get_feature(&self, fid: u64) -> Result<Feature<'_>, anyhow::Error> {
        self.layer
            .feature(fid)
            .context(format!("failed to retrieve feature with id: {}", fid))
    }

    pub fn build_turbines_geom_input_dto(
        &self,
        hub_height_default_mm: Option<u32>,
        blade_length_default_mm: Option<u32>,
        turbine_number_field: Option<String>,
        blade_length_field: Option<String>,
        hub_height_field: Option<String>,
    ) -> Result<TurbinesGeomInputDTO, anyhow::Error> {
        let mut turbine_builder = LayoutBuilder::new(
            hub_height_default_mm,
            blade_length_default_mm,
            turbine_number_field,
            blade_length_field,
            hub_height_field,
        );
        for fid in self.valid_feature_ids.iter() {
            let feature = self.get_feature(*fid)?;
            turbine_builder.add_feature(feature)?;
        }
        Ok(turbine_builder.turbines())
    }
}
