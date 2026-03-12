use std::collections::HashMap;

use crate::{
    helpers::{XmlNode, parse_xml_to_tree},
    layer::{
        DataSource, MapLayer, PgConfig, PgDataSource, PgSource, PgTable, QgisMapLayerBuilder,
        SslMode, WkbType,
    },
    srs::SpatialRefSys,
};

pub fn normalize_uuids(xml: &str) -> String {
    let mut result = String::new();
    let mut inside_braces = false;

    for ch in xml.chars() {
        match ch {
            '{' => {
                result.push_str("");

                inside_braces = true;
            }
            '}' => {
                inside_braces = false;
            }
            _ if !inside_braces => {
                result.push(ch);
            }
            _ => {} // Skip characters inside braces
        }
    }
    result
}

pub fn generate_pg_datasource() -> PgDataSource {
    PgDataSource {
        pg_config: PgConfig {
            db_name: "geodata_local".into(),
            host: "localhost".into(),
            port: 5432,
            sslmode: SslMode::Disable,
        },
        key: "id".into(),
        srid: Some(4326),
        r#type: Some(WkbType::MultiPolygon),
        checkPrimaryKeyUnicity: 1,
        source: PgSource::PgTable(PgTable {
            schema: "public".into(),
            table_name: "project_primary_boundaries".into(),
        }),
        geometry_col: "geom".into(),
        authcfg: Some("hbolxlf".into()),
    }
}

pub fn generate_pg_map_layer(layer_name: String) -> MapLayer {
    let builder = QgisMapLayerBuilder {
        layer_name,
        legend_text: None,
        include_on_legend: true,
        datasource: DataSource::Postgres(generate_pg_datasource()),
        srs: Some(SpatialRefSys::wgs84()),
    };
    builder.build_vector(WkbType::MultiPolygon)
}

pub fn xml_comparison(input: &str, expected: &str, skip_config: Option<&[SkipNode]>) {
    let mut input =
        parse_xml_to_tree(&normalize_uuids(input)).expect("failed to parse input xml to xml tree");
    let mut expected = parse_xml_to_tree(&normalize_uuids(expected))
        .expect("failed to parse expected xml to xml tree");

    xml_comparison_recursive(
        &mut input,
        &mut expected,
        0,
        &[],
        skip_config.unwrap_or(&[]),
    );
}

#[derive(Debug)]
pub struct SkipNode {
    pub node_name: String,
    pub attributes_to_skip: Vec<String>,
    pub children_to_skip: Vec<String>,
    pub skip_text: bool,
}

fn xml_comparison_recursive(
    input: &mut XmlNode,
    expected: &mut XmlNode,
    depth: usize,
    path: &[String],
    skip_config: &[SkipNode],
) {
    let current_path = [path, &[input.name.clone()]].concat();
    let location = format!(
        "\n\nlocation\ndepth: {}\npath: {}\n",
        depth,
        current_path.join(" > ")
    );

    let attributes_to_skip;
    let children_to_skip;
    let skip_text;

    match skip_config
        .iter()
        .find(|config| config.node_name == input.name)
    {
        Some(node) => {
            attributes_to_skip = node.attributes_to_skip.clone();
            children_to_skip = node.children_to_skip.clone();
            skip_text = node.skip_text;
        }
        None => {
            attributes_to_skip = Vec::new();
            children_to_skip = Vec::new();
            skip_text = false;
        }
    }

    // println!("{}", location);
    assert_eq!(
        input.name, expected.name,
        "nodes have different names, input: {}, expected: {}{}",
        input.name, expected.name, location
    );

    let expected_non_skipped_attributes: HashMap<_, _> = expected
        .attributes
        .iter()
        .filter(|(key, _)| !attributes_to_skip.contains(key))
        .collect();
    let mut expected_skipped_attributes: Vec<_> = expected
        .attributes
        .iter()
        .filter(|(key, _)| attributes_to_skip.contains(key))
        .collect();

    let mut expected_non_skipped_attributes_sorted: Vec<_> = expected_non_skipped_attributes
        .iter()
        .filter(|(key, _)| !attributes_to_skip.contains(key))
        .collect();

    let mut input_attributes_sorted: Vec<_> = input
        .attributes
        .iter()
        .filter(|(key, _)| !attributes_to_skip.contains(key))
        .collect();
    input_attributes_sorted.sort();
    expected_non_skipped_attributes_sorted.sort();
    expected_skipped_attributes.sort();

    assert_eq!(
        input_attributes_sorted.len(),
        expected_non_skipped_attributes.len(),
        "different number of attributes for '{}':\ninput has {}:\n{}\nexpected has {}:\n{}\n{}",
        input.name,
        input_attributes_sorted.len(),
        input_attributes_sorted
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", "),
        expected_non_skipped_attributes.len(),
        expected_non_skipped_attributes_sorted
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", "),
        location
    );

    for (key, value) in input_attributes_sorted.iter() {
        let expected_value = expected_non_skipped_attributes.get(key).unwrap_or_else(|| {
            panic!(
                "expected does not contain attribute '{key}' found in input node '{}'{}",
                input.name, location
            )
        });
        assert_eq!(
            value, expected_value,
            "input node, '{}', value, '{}', for attribute, '{}', does not match expected value '{}'{}",
            input.name, value, key, expected_value, location
        );
    }

    for key in expected.attributes.keys() {
        if !attributes_to_skip.contains(key) {
            assert!(
                input.attributes.contains_key(key),
                "input node, '{}', does not contain attribute '{}' found in expected{}",
                input.name,
                key,
                location
            )
        }
    }

    if !skip_text {
        assert_eq!(
            input.text, expected.text,
            "input node, '{}', has different text from expected.\nInput:\n{}\nExpected:\n{}\n{}",
            input.name, input.text, expected.text, location
        );
    }
    input.children.sort();
    expected.children.sort();

    let input_non_skipped: Vec<_> = input
        .children
        .iter()
        .filter(|child| !children_to_skip.contains(&child.name))
        .collect();
    let expected_non_skipped: Vec<_> = expected
        .children
        .iter()
        .filter(|child| !children_to_skip.contains(&child.name))
        .collect();

    assert_eq!(
        input_non_skipped.len(),
        expected_non_skipped.len(),
        "different number of children for '{}':\n\ninput has {}:\n{}\nexpected has {}:\n{}{}",
        input.name,
        input_non_skipped.len(),
        input_non_skipped
            .iter()
            .map(|node| node.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        expected_non_skipped.len(),
        expected_non_skipped
            .iter()
            .map(|node| node.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        location
    );

    println!(
        "input children: {}",
        input
            .children
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!(
        "expected children to include: {}",
        expected
            .children
            .iter()
            .filter(|c| !children_to_skip.contains(&c.name.to_owned()))
            .map(|c| c.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );

    println!(
        "expected children to skip: {}",
        expected
            .children
            .iter()
            .filter(|c| children_to_skip.contains(&c.name.to_owned()))
            .map(|c| c.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Filter out skipped nodes before comparing children
    let mut input_filtered: Vec<_> = input
        .children
        .iter_mut()
        .filter(|child| !children_to_skip.contains(&child.name))
        .collect();
    let mut expected_filtered: Vec<_> = expected
        .children
        .iter_mut()
        .filter(|child| !children_to_skip.contains(&child.name))
        .collect();

    for (input_child, expected_child) in input_filtered.iter_mut().zip(expected_filtered.iter_mut())
    {
        xml_comparison_recursive(
            input_child,
            expected_child,
            depth + 1,
            &current_path,
            skip_config,
        );
    }
}

pub fn layout_skip_config() -> [SkipNode; 6] {
    [
        SkipNode {
            node_name: "Layout".into(),
            attributes_to_skip: vec!["worldFileMap".into()],
            children_to_skip: vec!["Snapper".into(), "customproperties".into()],
            skip_text: false,
        },
        SkipNode {
            node_name: "LayoutObject".into(),
            attributes_to_skip: vec![],
            children_to_skip: vec!["customproperties".into()],
            skip_text: false,
        },
        SkipNode {
            node_name: "Option".into(),
            attributes_to_skip: vec![],
            children_to_skip: vec!["Option".into()],
            skip_text: false,
        },
        SkipNode {
            node_name: "LayoutItem".into(),
            attributes_to_skip: vec![
                "position".into(),
                "positionOnPage".into(),
                "size".into(),
                "endMarkerFile".into(),
                "startMarkerFile".into(),
            ],
            children_to_skip: vec![],
            skip_text: false,
        },
        SkipNode {
            node_name: "AtlasMap".into(),
            attributes_to_skip: vec!["margin".into()],
            children_to_skip: vec![],
            skip_text: false,
        },
        SkipNode {
            node_name: "Extent".into(),
            attributes_to_skip: vec!["xmax".into(), "xmin".into(), "ymin".into(), "ymax".into()],
            children_to_skip: vec![],
            skip_text: false,
        },
    ]
}

pub fn map_layer_skip_config() -> [SkipNode; 5] {
    [
        SkipNode {
            node_name: "maplayer".into(),
            children_to_skip: vec![
                "aliases".into(),
                "attributeactions".into(),
                "attributetableconfig".into(),
                "temporal".into(),
                "elevation".into(),
                "renderer-v2".into(),
                "selection".into(),
                "customproperties".into(),
                "blendMode".into(),
                "featureBlendMode".into(),
                "layerOpacity".into(),
                "geometryOptions".into(),
                "legend".into(),
                "referencedLayers".into(),
                "fieldConfiguration".into(),
                "splitPolicies".into(),
                "defaults".into(),
                "constraints".into(),
                "constraintExpressions".into(),
                "conditionalstyles".into(),
                "storedexpressions".into(),
                "editform".into(),
                "editforminit".into(),
                "editforminitcodesource".into(),
                "editforminitfilepath".into(),
                "editforminitcode".into(),
                "featformsuppress".into(),
                "editorlayout".into(),
                "editable".into(),
                "labelOnTop".into(),
                "reuseLastValue".into(),
                "dataDefinedFieldProperties".into(),
                "widgets".into(),
                "previewExpression".into(),
                "mapTip".into(),
                "extent".into(),
                "wgs84extent".into(),
            ],
            attributes_to_skip: Vec::new(),
            skip_text: false,
        },
        SkipNode {
            node_name: "id".into(),
            children_to_skip: Vec::new(),
            attributes_to_skip: Vec::new(),
            skip_text: true,
        },
        SkipNode {
            node_name: "item".into(),
            children_to_skip: Vec::new(),
            attributes_to_skip: Vec::new(),
            skip_text: true,
        },
        SkipNode {
            node_name: "layer-tree-layer".into(),
            children_to_skip: Vec::new(),
            attributes_to_skip: vec!["id".into()],
            skip_text: false,
        },
        SkipNode {
            node_name: "resourceMetadata".into(),
            children_to_skip: vec!["crs".into()],
            attributes_to_skip: vec![],
            skip_text: false,
        },
    ]
}

pub fn minimal_project_skip_config() -> [SkipNode; 7] {
    [
        SkipNode {
            node_name: "qgis".into(),
            attributes_to_skip: vec![
                "saveUserFull".into(),
                "saveUser".into(),
                "saveDateTime".into(),
            ],
            children_to_skip: vec![
                "Annotations".into(),
                "Bookmarks".into(),
                "ElevationProperties".into(),
                "ProjectDisplaySettings".into(),
                "ProjectGpsSettings".into(),
                "ProjectStyleSettings".into(),
                "ProjectTimeSettings".into(),
                "ProjectViewSettings".into(),
                "Sensors".into(),
                "dataDefinedServerProperties".into(),
                "elevation-shading-renderer".into(),
                "homePath".into(),
                "layerorder".into(),
                "legend".into(),
                "main-annotation-layer".into(),
                "mapViewDocks".into(),
                "mapViewDocks3D".into(),
                "polymorphicRelations".into(),
                "projectFlags".into(),
                "projectMetadata".into(),
                "projectModels".into(),
                "relations".into(),
                "snapping-settings".into(),
                "transaction".into(),
                "transformContext".into(),
                "visibility-presets".into(),
                "properties".into(),
            ],
            skip_text: false,
        },
        SkipNode {
            node_name: "spatialrefsys".into(),
            attributes_to_skip: vec!["nativeFormat".into()],
            children_to_skip: vec!["wkt".into()],
            skip_text: false,
        },
        SkipNode {
            node_name: "xmax".into(),
            attributes_to_skip: Vec::new(),
            children_to_skip: Vec::new(),
            skip_text: true,
        },
        SkipNode {
            node_name: "xmin".into(),
            attributes_to_skip: Vec::new(),
            children_to_skip: Vec::new(),
            skip_text: true,
        },
        SkipNode {
            node_name: "ymax".into(),
            attributes_to_skip: Vec::new(),
            children_to_skip: Vec::new(),
            skip_text: true,
        },
        SkipNode {
            node_name: "ymin".into(),
            attributes_to_skip: Vec::new(),
            children_to_skip: Vec::new(),
            skip_text: true,
        },
        SkipNode {
            node_name: "properties".into(),
            attributes_to_skip: Vec::new(),
            children_to_skip: vec![
                "Digitizing".into(),
                "Gui".into(),
                "Legend".into(),
                "Measure".into(),
                "Measurement".into(),
                "PAL".into(),
                "Paths".into(),
                "PositionPrecision".into(),
            ],
            skip_text: false,
        },
    ]
}
