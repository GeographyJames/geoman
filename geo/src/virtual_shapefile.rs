#![allow(unused)]

use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use anyhow::{Context, anyhow};
use gdal::{
    vector::LayerAccess,
    vsi::{self, get_vsi_mem_file_bytes_owned},
};
use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
};
use uuid::Uuid;

use utils::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum ShapefileError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("incorrect files: {0}")]
    IncorrectFiles(String),
    #[error(transparent)]
    InvalidData(anyhow::Error),
}

#[derive(MultipartForm)]
pub struct ShapefileForm {
    pub shp: TempFile,
    pub dbf: TempFile,
    pub shx: TempFile,
    pub prj: TempFile,
}

#[derive(Debug)]
pub struct VirtualShapefile {
    shp: VirtualFile,
    _dbf: VirtualFile,
    _shx: VirtualFile,
    _prj: VirtualFile,
}

impl VirtualShapefile {
    pub fn new(filename: String, data: ShapefileData) -> Result<Self, ShapefileError> {
        Ok(Self {
            shp: VirtualFile::new(&format!("{filename}.shp"), data.shp)?,
            _dbf: VirtualFile::new(&format!("{filename}.dbf"), data.dbf)?,
            _shx: VirtualFile::new(&format!("{filename}.shx"), data.shx)?,
            _prj: VirtualFile::new(&format!("{filename}.prj"), data.prj)?,
        })
    }
    pub fn path(&self) -> &String {
        &self.shp.0
    }
}

impl TryInto<gdal::Dataset> for ShapefileData {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<gdal::Dataset, Self::Error> {
        let file_id = Uuid::new_v4();
        let _virtual_shapefile = VirtualShapefile::new(file_id.to_string(), self)
            .context("failed to create virtual shapefile")?;

        let ds = gdal::Dataset::open(format!("/vsimem/{}.shp", file_id))
            .context("failed to open shapefile")
            .context("failed to open data")?;
        let layer = ds.layers().next().context("no layers on shapefile")?;
        // Gotta leave this line in or things break! :/
        let _srs = layer.spatial_ref().context("no spatial ref on layer!!!")?;
        Ok(ds)
    }
}

pub struct ShapefileData {
    pub shp: Vec<u8>,
    pub dbf: Vec<u8>,
    pub shx: Vec<u8>,
    pub prj: Vec<u8>,
}

impl std::fmt::Debug for ShapefileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ShapefileData {
    pub fn try_from_gdal_vsi_mem_file<P>(path: P) -> Result<Self, ShapefileError>
    where
        P: AsRef<Path> + Clone,
    {
        let shp = path.as_ref();
        let mut shx = PathBuf::from(path.as_ref());
        shx.set_extension("shx");
        let mut dbf = PathBuf::from(path.as_ref());
        dbf.set_extension("dbf");
        let mut prj = PathBuf::from(path.as_ref());
        prj.set_extension("prj");

        Ok(ShapefileData {
            shp: get_vsi_mem_file_bytes_owned(shp)
                .context("failed to read im mem file to bytes")?,
            shx: get_vsi_mem_file_bytes_owned(shx)
                .context("failed to read im mem file to bytes")?,
            dbf: get_vsi_mem_file_bytes_owned(dbf)
                .context("failed to read im mem file to bytes")?,
            prj: get_vsi_mem_file_bytes_owned(prj)
                .context("failed to read im mem file to bytes")?,
        })
    }
}

impl TryFrom<HashMap<String, Vec<u8>>> for ShapefileData {
    type Error = ShapefileError;

    fn try_from(mut data: HashMap<String, Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(ShapefileData {
            shp: data.remove("shp").ok_or(ShapefileError::IncorrectFiles(
                "missing .shp file".to_string(),
            ))?,
            dbf: data.remove("dbf").ok_or(ShapefileError::IncorrectFiles(
                "missing .dbf file".to_string(),
            ))?,
            shx: data.remove("shx").ok_or(ShapefileError::IncorrectFiles(
                "missing .shx file".to_string(),
            ))?,
            prj: data.remove("prj").ok_or(ShapefileError::IncorrectFiles(
                "missing .prj file".to_string(),
            ))?,
        })
    }
}

impl ShapefileData {
    pub fn try_from_temp_files(
        mut shp: TempFile,
        mut dbf: TempFile,
        mut shx: TempFile,
        mut prj: TempFile,
    ) -> Result<Self, anyhow::Error> {
        let mut shp_data = Vec::new();
        let mut dbf_data = Vec::new();
        let mut shx_data = Vec::new();
        let mut prj_data = Vec::new();
        dbf.file
            .read_to_end(&mut dbf_data)
            .context("failed to read dbf")?;
        shx.file
            .read_to_end(&mut shx_data)
            .context("failed to read shx")?;
        prj.file
            .read_to_end(&mut prj_data)
            .context("failed to read prj")?;
        shp.file
            .read_to_end(&mut shp_data)
            .context("failed to read shp")?;
        Ok(ShapefileData {
            shp: shp_data,
            dbf: dbf_data,
            shx: shx_data,
            prj: prj_data,
        })
    }
}

impl TryFrom<ShapefileForm> for ShapefileData {
    type Error = ShapefileError;
    fn try_from(mut form: ShapefileForm) -> Result<Self, Self::Error> {
        let mut shp = Vec::new();
        let mut dbf = Vec::new();
        let mut shx = Vec::new();
        let mut prj = Vec::new();
        form.dbf
            .file
            .read_to_end(&mut dbf)
            .context("failed to read dbf")?;
        form.shx
            .file
            .read_to_end(&mut shx)
            .context("failed to read shx")?;
        form.prj
            .file
            .read_to_end(&mut prj)
            .context("failed to read prj")?;
        form.shp
            .file
            .read_to_end(&mut shp)
            .context("failed to read shp")?;
        Ok(ShapefileData { shp, dbf, shx, prj })
    }
}

#[derive(Debug)]
pub struct VirtualFile(String);

impl VirtualFile {
    const MEM_FILE_PATH: &str = "/vsimem";
    pub fn new(filename: &str, data: Vec<u8>) -> Result<Self, anyhow::Error> {
        if data.is_empty() {
            return Err(anyhow!(
                "failed to create virtual file '{}': empty data vector",
                filename
            ));
        }
        let path = format!("{}/{}", Self::MEM_FILE_PATH, filename);
        vsi::create_mem_file(&path, data)
            .context(format!("failed to create virtual file: '{}'", path))?;
        Ok(Self(path))
    }
}

impl Drop for VirtualFile {
    fn drop(&mut self) {
        if let Err(e) = vsi::unlink_mem_file(&self.0) {
            panic!("failed to drop virtual file: {}", e)
        };
    }
}

/// Extract the EPSG code from a .prj WKT string.
/// Creates a minimal virtual shapefile with the .prj content so GDAL's shapefile driver
/// can apply its full CRS matching logic (including FindMatches), which handles
/// Esri-style WKT that lacks AUTHORITY tags.
pub struct CrsInfo {
    pub srid: i32,
    pub name: Option<String>,
}

pub fn get_epsg_from_prj(prj: &str) -> Result<CrsInfo, ShapefileError> {
    use gdal::vector::LayerOptions;

    let file_id = Uuid::new_v4();
    let filename = format!("/vsimem/{}", file_id);

    // Create a minimal empty shapefile with a layer so GDAL writes valid .shp/.shx/.dbf
    let driver = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
        .context("failed to get shapefile driver")
        .map_err(ShapefileError::UnexpectedError)?;
    let mut ds = driver
        .create_vector_only(format!("{filename}.shp"))
        .context("failed to create virtual dataset")
        .map_err(ShapefileError::UnexpectedError)?;
    ds.create_layer(LayerOptions {
        name: "dummy",
        ty: gdal::vector::OGRwkbGeometryType::wkbPoint,
        ..Default::default()
    })
    .context("failed to create layer")
    .map_err(ShapefileError::UnexpectedError)?;
    ds.flush_cache()
        .context("failed to flush dataset")
        .map_err(ShapefileError::UnexpectedError)?;
    ds.close()
        .context("failed to close dataset")
        .map_err(ShapefileError::UnexpectedError)?;

    // Write the real .prj content over the generated one
    let prj_path = format!("{filename}.prj");
    let _ = vsi::unlink_mem_file(&prj_path);
    vsi::create_mem_file(&prj_path, prj.as_bytes().to_vec())
        .context("failed to create virtual .prj file")
        .map_err(ShapefileError::UnexpectedError)?;

    // Re-open the shapefile - GDAL's driver will read our .prj and apply full CRS matching
    let mut ds = gdal::Dataset::open(format!("{filename}.shp"))
        .context("failed to open virtual shapefile")
        .map_err(ShapefileError::UnexpectedError)?;
    let layer = ds
        .layers()
        .next()
        .context("no layers on virtual shapefile")
        .map_err(ShapefileError::UnexpectedError)?;
    let srs = layer
        .spatial_ref()
        .context("no spatial reference on layer")
        .map_err(ShapefileError::InvalidData)?;
    let code = srs
        .auth_code()
        .context("failed to extract EPSG code")
        .map_err(ShapefileError::InvalidData)?;
    let name = srs.name();

    // Cleanup virtual files
    let _ = vsi::unlink_mem_file(format!("{filename}.shp"));
    let _ = vsi::unlink_mem_file(format!("{filename}.shx"));
    let _ = vsi::unlink_mem_file(format!("{filename}.dbf"));
    let _ = vsi::unlink_mem_file(&prj_path);

    Ok(CrsInfo { srid: code, name })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use claims::{assert_err, assert_ok};
    use gdal::{Dataset, vector::LayerAccess};

    #[test]
    fn get_epsg_from_esri_style_prj_works() {
        let prj = fs::read_to_string("../test-data/shapefiles/3_valid_polygon_osgb36.prj")
            .expect("failed to read .prj file");
        let crs = get_epsg_from_prj(&prj).expect("failed to extract EPSG code");
        assert_eq!(crs.srid, 27700);
        assert_eq!(crs.name.as_deref(), Some("OSGB36 / British National Grid"));
    }

    #[test]
    fn get_epsg_from_standard_wkt_works() {
        let srs =
            gdal::spatial_ref::SpatialRef::from_epsg(4326).expect("failed to create spatial ref");
        let wkt = srs.to_wkt().expect("failed to convert to WKT");
        let crs = get_epsg_from_prj(&wkt).expect("failed to extract EPSG code");
        assert_eq!(crs.srid, 4326);
    }

    #[test]
    fn virtual_file_works() {
        assert_ok!(VirtualFile::new("test_file.text", Vec::from([1])));
    }
    #[test]
    fn virtual_file_returns_error_for_empty_data_vector() {
        assert_err!(VirtualFile::new("test_file.text", Vec::new()));
    }

    #[test]
    #[should_panic]
    fn panics_when_attetmpting_to_drop_non_existent_file() {
        let _f = VirtualFile("/vsimem/non-existent.txt".to_string());
    }

    #[test]
    fn virtual_shapefile_works_with_dummy_data() {
        let data = ShapefileData {
            shp: Vec::from([1]),
            dbf: Vec::from([1]),
            shx: Vec::from([1]),
            prj: Vec::from([1]),
        };
        assert_ok!(VirtualShapefile::new("test_file".to_string(), data));
    }

    #[test]
    fn virtual_shapefile_works_with_real_data() {
        let path = "../test-data/shapefiles/3_valid_polygon_osgb36";
        let data = ShapefileData {
            shp: fs::read(format!("{path}.shp")).unwrap(),
            dbf: fs::read(format!("{path}.dbf")).unwrap(),
            shx: fs::read(format!("{path}.shx")).unwrap(),
            prj: fs::read(format!("{path}.prj")).unwrap(),
        };
        let v_shapefile = VirtualShapefile::new("test_file".to_string(), data)
            .expect("failed to create virtual shapefile");
        let ds = Dataset::open(v_shapefile.path()).expect("failed to open file");
        let first_layer = ds.layers().next().expect("couldn't access layer");
        assert_eq!(first_layer.feature_count(), 3);
    }
}
