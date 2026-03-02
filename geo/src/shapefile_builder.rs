use anyhow::Context;
use domain::enums::GeometryType;
use gdal::{
    spatial_ref::SpatialRef,
    vector::{Defn, Feature, LayerAccess, OGRFieldType, OGRwkbGeometryType},
    vsi::{self, get_vsi_mem_file_bytes_owned},
};
use uuid::Uuid;

use crate::virtual_shapefile::ShapefileError;

pub struct TurbineEntry {
    pub id: i32,
    pub turbine_number: i32,
    pub hub_height_m: Option<f64>,
    pub rotor_diameter_m: Option<f64>,
    pub geom_wkb: Vec<u8>,
}

pub fn build_project_feature_shapefile(
    srid: u32,
    geom_type: GeometryType,
    layer_name: &str,
    geom_wkb: &[u8],
) -> Result<Vec<u8>, ShapefileError> {
    let vsimem_path = format!("/vsimem/{}.shz", Uuid::new_v4());

    let driver = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
        .context("failed to get gdal shapefile driver")?;
    let mut dataset = driver
        .create_vector_only(&vsimem_path)
        .context("failed to create dataset")?;
    let srs = SpatialRef::from_epsg(srid).context("failed to get spatial ref")?;
    let mut layer = dataset
        .create_layer(gdal::vector::LayerOptions {
            name: layer_name,
            srs: Some(&srs),
            ty: geom_type.into(),
            options: None,
        })
        .context("failed to create layer")?;
    let geom =
        gdal::vector::Geometry::from_wkb(geom_wkb).context("failed to parse geometry from wkb")?;
    layer
        .create_feature(geom)
        .context("failed to add feature to layer")?;

    finalize_and_read(&vsimem_path, dataset)
}

pub fn build_turbine_layout_shapefile(
    srid: u32,
    layer_name: &str,
    turbines: &[TurbineEntry],
) -> Result<Vec<u8>, ShapefileError> {
    let vsimem_path = format!("/vsimem/{}.shz", Uuid::new_v4());

    let driver = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
        .context("failed to get gdal shapefile driver")?;
    let mut dataset = driver
        .create_vector_only(&vsimem_path)
        .context("failed to create dataset")?;
    let srs = SpatialRef::from_epsg(srid).context("failed to get spatial ref")?;
    let layer = dataset
        .create_layer(gdal::vector::LayerOptions {
            name: layer_name,
            srs: Some(&srs),
            ty: OGRwkbGeometryType::wkbPoint,
            options: None,
        })
        .context("failed to create layer")?;

    let fields = [
        ("id", OGRFieldType::OFTInteger),
        ("turb_num", OGRFieldType::OFTInteger),
        ("hub_ht_m", OGRFieldType::OFTReal),
        ("rotor_d_m", OGRFieldType::OFTReal),
    ];
    layer
        .create_defn_fields(&fields)
        .context("failed to create fields")?;
    let defn = Defn::from_layer(&layer);
    let id_idx = defn.field_index("id").context("id field not found")?;
    let turb_num_idx = defn
        .field_index("turb_num")
        .context("turb_num field not found")?;
    let hub_idx = defn
        .field_index("hub_ht_m")
        .context("hub_ht_m field not found")?;
    let rotor_idx = defn
        .field_index("rotor_d_m")
        .context("rotor_d_m field not found")?;

    for turbine in turbines {
        let geom = gdal::vector::Geometry::from_wkb(&turbine.geom_wkb)
            .context("failed to parse geometry from wkb")?;
        let mut feature = Feature::new(&defn).context("failed to create feature")?;
        feature
            .set_geometry(geom)
            .context("failed to set geometry")?;
        feature
            .set_field_integer(id_idx, turbine.id)
            .context("failed to set id field")?;
        feature
            .set_field_integer(turb_num_idx, turbine.turbine_number)
            .context("failed to set turb_num field")?;
        if let Some(v) = turbine.hub_height_m {
            feature
                .set_field_double(hub_idx, v)
                .context("failed to set hub_ht_m field")?;
        }
        if let Some(v) = turbine.rotor_diameter_m {
            feature
                .set_field_double(rotor_idx, v)
                .context("failed to set rotor_d_m field")?;
        }
        feature
            .create(&layer)
            .context("failed to add feature to layer")?;
    }

    finalize_and_read(&vsimem_path, dataset)
}

fn finalize_and_read(
    vsimem_path: &str,
    mut dataset: gdal::Dataset,
) -> Result<Vec<u8>, ShapefileError> {
    dataset.flush_cache().context("failed to flush cache")?;
    dataset.close().context("failed to close dataset")?;
    let content =
        get_vsi_mem_file_bytes_owned(vsimem_path).context("failed to read shapefile bytes")?;
    let _ = vsi::unlink_mem_file(vsimem_path);
    Ok(content)
}
