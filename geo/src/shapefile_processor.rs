use anyhow::Context;
use gdal::vector::{Geometry, LayerAccess, OGRwkbGeometryType, geometry_type_to_name};

#[derive(thiserror::Error, Debug)]
pub enum ProcessingError {
    #[error("no layers found in dataset")]
    NoLayers,
    #[error("no features with geometry found in layer")]
    NoFeaturesWithGeometry,
    #[error("incompatible geometry type on feature {index}: expected {expected} but found {found}")]
    IncompatibleType {
        index: usize,
        expected: String,
        found: String,
    },
    #[error("expected single geometry type {expected} but found {count} geometries")]
    MultipleGeometries { expected: String, count: usize },
    #[error("unsupported geometry type: {0}")]
    UnsupportedGeometryType(String),
    #[error("feature with invalid geometry. Try fixing the geometries and try again.")]
    InvalidGeometry { index: usize },
    #[error("merged geometry is invalid")]
    InvalidMergedGeometry,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub fn merge_geometries(
    dataset: &gdal::Dataset,
    expected_type: OGRwkbGeometryType::Type,
) -> Result<Geometry, ProcessingError> {
    let mut layer = dataset.layers().next().ok_or(ProcessingError::NoLayers)?;
    let TypeInfo { single, multi } = compatible_types(expected_type)?;
    let is_single = expected_type == single;
    let mut merged = Geometry::empty(multi).context("failed to create empty multi-geometry")?;

    for (index, feature) in layer.features().enumerate() {
        let Some(geom) = feature.geometry() else {
            continue;
        };
        if geom.is_empty() {
            continue;
        }
        if !geom.is_valid() {
            return Err(ProcessingError::InvalidGeometry { index });
        }
        let geom_type = geom.geometry_type();

        if geom_type == single {
            merged
                .add_geometry(geom.clone())
                .context("failed to add geometry")?;
        } else if geom_type == multi {
            for i in 0..geom.geometry_count() {
                merged
                    .add_geometry(geom.get_geometry(i).clone())
                    .context("failed to add sub-geometry")?;
            }
        } else {
            return Err(ProcessingError::IncompatibleType {
                index,
                expected: geometry_type_to_name(expected_type),
                found: geometry_type_to_name(geom_type),
            });
        }
    }

    if merged.is_empty() {
        return Err(ProcessingError::NoFeaturesWithGeometry);
    }

    if !merged.is_valid() {
        return Err(ProcessingError::InvalidMergedGeometry);
    }

    if is_single && merged.geometry_count() > 1 {
        return Err(ProcessingError::MultipleGeometries {
            expected: geometry_type_to_name(expected_type),
            count: merged.geometry_count(),
        });
    }

    if is_single {
        return Ok(merged.get_geometry(0).clone());
    }

    Ok(merged)
}

struct TypeInfo {
    single: OGRwkbGeometryType::Type,
    multi: OGRwkbGeometryType::Type,
}

fn compatible_types(expected_type: OGRwkbGeometryType::Type) -> Result<TypeInfo, ProcessingError> {
    match expected_type {
        OGRwkbGeometryType::wkbPoint | OGRwkbGeometryType::wkbMultiPoint => Ok(TypeInfo {
            single: OGRwkbGeometryType::wkbPoint,
            multi: OGRwkbGeometryType::wkbMultiPoint,
        }),
        OGRwkbGeometryType::wkbLineString | OGRwkbGeometryType::wkbMultiLineString => {
            Ok(TypeInfo {
                single: OGRwkbGeometryType::wkbLineString,
                multi: OGRwkbGeometryType::wkbMultiLineString,
            })
        }
        OGRwkbGeometryType::wkbPolygon | OGRwkbGeometryType::wkbMultiPolygon => Ok(TypeInfo {
            single: OGRwkbGeometryType::wkbPolygon,
            multi: OGRwkbGeometryType::wkbMultiPolygon,
        }),
        ty => Err(ProcessingError::UnsupportedGeometryType(
            geometry_type_to_name(ty),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gdal::vector::{LayerOptions, OGRwkbGeometryType};

    fn create_test_dataset(
        geom_type: OGRwkbGeometryType::Type,
        crs: u32,
    ) -> (gdal::Dataset, String) {
        let filename = format!("/vsimem/{}.shp", uuid::Uuid::new_v4());
        let mut dataset = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
            .expect("failed to get shapefile driver")
            .create_vector_only(&filename)
            .expect("failed to create dataset");
        dataset
            .create_layer(LayerOptions {
                name: "test",
                options: None,
                ty: geom_type,
                srs: Some(
                    &gdal::spatial_ref::SpatialRef::from_epsg(crs).expect("failed to create srs"),
                ),
            })
            .expect("failed to create layer");
        (dataset, filename)
    }

    #[test]
    fn compatible_types_returns_correct_types_for_multipoint() {
        let info = compatible_types(OGRwkbGeometryType::wkbMultiPoint).unwrap();
        assert_eq!(info.single, OGRwkbGeometryType::wkbPoint);
        assert_eq!(info.multi, OGRwkbGeometryType::wkbMultiPoint);
    }

    #[test]
    fn compatible_types_returns_error_for_unsupported_type() {
        let result = compatible_types(OGRwkbGeometryType::wkbGeometryCollection);
        assert!(matches!(
            result,
            Err(ProcessingError::UnsupportedGeometryType(_))
        ));
    }

    #[test]
    fn merges_polygons_into_multipolygon() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbPolygon, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            let p1 = Geometry::from_wkt("POLYGON((0 0, 1 0, 1 1, 0 1, 0 0))").unwrap();
            let p2 = Geometry::from_wkt("POLYGON((2 2, 3 2, 3 3, 2 3, 2 2))").unwrap();
            layer.create_feature(p1).unwrap();
            layer.create_feature(p2).unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbMultiPolygon).unwrap();
        assert_eq!(result.geometry_type(), OGRwkbGeometryType::wkbMultiPolygon);
        assert_eq!(result.geometry_count(), 2);
    }

    #[test]
    fn merges_mixed_polygon_and_multipolygon() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbMultiPolygon, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            let multi = Geometry::from_wkt(
                "MULTIPOLYGON(((0 0, 1 0, 1 1, 0 1, 0 0)),((2 2, 3 2, 3 3, 2 3, 2 2)))",
            )
            .unwrap();
            let single = Geometry::from_wkt("POLYGON((4 4, 5 4, 5 5, 4 5, 4 4))").unwrap();
            layer.create_feature(multi).unwrap();
            layer.create_feature(single).unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbMultiPolygon).unwrap();
        assert_eq!(result.geometry_type(), OGRwkbGeometryType::wkbMultiPolygon);
        assert_eq!(result.geometry_count(), 3);
    }

    #[test]
    fn merges_points_into_multipoint() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbPoint, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            layer
                .create_feature(Geometry::from_wkt("POINT(1 1)").unwrap())
                .unwrap();
            layer
                .create_feature(Geometry::from_wkt("POINT(2 2)").unwrap())
                .unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbMultiPoint).unwrap();
        assert_eq!(result.geometry_type(), OGRwkbGeometryType::wkbMultiPoint);
        assert_eq!(result.geometry_count(), 2);
    }

    #[test]
    fn merges_linestrings_into_multilinestring() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbLineString, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            layer
                .create_feature(Geometry::from_wkt("LINESTRING(0 0, 1 1)").unwrap())
                .unwrap();
            layer
                .create_feature(Geometry::from_wkt("LINESTRING(2 2, 3 3)").unwrap())
                .unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbMultiLineString).unwrap();
        assert_eq!(
            result.geometry_type(),
            OGRwkbGeometryType::wkbMultiLineString
        );
        assert_eq!(result.geometry_count(), 2);
    }

    #[test]
    fn rejects_incompatible_geometry_type() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbPoint, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            layer
                .create_feature(Geometry::from_wkt("POINT(1 1)").unwrap())
                .unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbMultiPolygon);
        assert!(matches!(
            result,
            Err(ProcessingError::IncompatibleType { .. })
        ));
    }

    #[test]
    fn returns_error_for_empty_layer() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbPolygon, 27700);

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbMultiPolygon);
        assert!(matches!(
            result,
            Err(ProcessingError::NoFeaturesWithGeometry)
        ));
    }

    #[test]
    fn returns_single_polygon_from_multipolygon_with_one_polygon() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbMultiPolygon, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            layer
                .create_feature(
                    Geometry::from_wkt("MULTIPOLYGON(((0 0, 1 0, 1 1, 0 1, 0 0)))").unwrap(),
                )
                .unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbPolygon).unwrap();
        assert_eq!(result.geometry_type(), OGRwkbGeometryType::wkbPolygon);
    }

    #[test]
    fn returns_single_polygon_when_expected_type_is_polygon() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbPolygon, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            layer
                .create_feature(Geometry::from_wkt("POLYGON((0 0, 10 0, 10 10, 0 10, 0 0),(2 2, 4 2, 4 4, 2 4, 2 2),(6 6, 8 6, 8 8, 6 8, 6 6))").unwrap())
                .unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbPolygon).unwrap();
        assert_eq!(result.geometry_type(), OGRwkbGeometryType::wkbPolygon);
    }

    #[test]
    fn returns_error_for_multiple_features_when_single_type_expected() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbPolygon, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            layer
                .create_feature(Geometry::from_wkt("POLYGON((0 0, 1 0, 1 1, 0 1, 0 0))").unwrap())
                .unwrap();
            layer
                .create_feature(Geometry::from_wkt("POLYGON((2 2, 3 2, 3 3, 2 3, 2 2))").unwrap())
                .unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbPolygon);
        assert!(matches!(
            result,
            Err(ProcessingError::MultipleGeometries { .. })
        ));
    }

    #[test]
    fn returns_single_point_when_expected_type_is_point() {
        let (dataset, _) = create_test_dataset(OGRwkbGeometryType::wkbPoint, 27700);
        {
            let mut layer = dataset.layer(0).unwrap();
            layer
                .create_feature(Geometry::from_wkt("POINT(1 1)").unwrap())
                .unwrap();
        }

        let result = merge_geometries(&dataset, OGRwkbGeometryType::wkbPoint).unwrap();
        assert_eq!(result.geometry_type(), OGRwkbGeometryType::wkbPoint);
    }
}
