use crate::enums::EPSGID;
use crate::helpers::unzip_content;
use crate::layer::{
    DataSource, PgSource, PgTable, QgisMapLayerBuilder, WkbType, XYZDataSource,
};

use crate::layout::components::{Color, LayoutItem, Position};
use crate::layout::{PageOrientation, PageSize, QgisLayoutBuilder, QgisLayoutMapBuilder};

use crate::project::{MapCanvas, ProjectRoot, QgisProjectBuilder};
use crate::srs::SpatialRefSys;
use crate::tests::test_helpers::{
    SkipNode, generate_pg_datasource, generate_pg_map_layer, layout_skip_config,
    map_layer_skip_config, minimal_project_skip_config, xml_comparison,
};

#[test]
fn build_minimal_qgs_xml_works() {
    let root = ProjectRoot::new_wgs84();

    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let input = &unzip_content(&project.content).expect("failed to unzip xml");
    let expected = include_str!("minimal_example_project.xml");
    let skip_config = minimal_project_skip_config();
    xml_comparison(input, expected, Some(&skip_config));
}

#[test]
fn build_minimal_qgs_with_layout_xml_works() {
    let layout = QgisLayoutBuilder {
        page_size: PageSize::A4(PageOrientation::Landscape).into(),
        layout_items: Vec::new(),
        layout_maps: Vec::new(),
        print_resolution: 300,
        name: "default-layout".to_string(),
    };

    let mut root = ProjectRoot::new_wgs84();
    root.add_layout(layout);

    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");

    let input = &unzip_content(&project.content).expect("failed to unzip xml");
    let expected = include_str!("minimal_example_project_with_layout.xml");

    let additional_skip_nodes = layout_skip_config();
    let skip_nodes: Vec<SkipNode> = minimal_project_skip_config()
        .into_iter()
        .chain(additional_skip_nodes)
        .collect();

    xml_comparison(input, expected, Some(&skip_nodes));
}

#[test]
fn layout_with_polyline() {
    let polyline_start = Position { x: 5., y: 5. };
    let polyline_end = Position { x: 105., y: 5. };
    let polyline = LayoutItem::single_line(polyline_start, polyline_end);

    let layout = QgisLayoutBuilder {
        page_size: PageSize::default().into(),
        layout_items: vec![polyline],
        layout_maps: vec![],
        print_resolution: 300,
        name: "default-layout".to_string(),
    };
    let mut root = ProjectRoot::new_wgs84();

    root.add_layout(layout);

    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let input = &unzip_content(&project.content).expect("failet to unzip xml");
    let expected = include_str!("layout_with_polyline.xml");
    let mut skip_config: Vec<SkipNode> = minimal_project_skip_config()
        .into_iter()
        .chain(layout_skip_config())
        .collect();
    skip_config.push(SkipNode {
        node_name: "node".into(),
        attributes_to_skip: vec!["x".into(), "y".into()],
        children_to_skip: vec![],
        skip_text: false,
    });
    println!("{}", input);
    xml_comparison(input, expected, Some(&skip_config));
}

#[test]
fn layout_with_text() {
    let text_item = LayoutItem::text("Lorem ipsum".into(), 10, None, Color::black());

    let layout = QgisLayoutBuilder {
        page_size: PageSize::A4(PageOrientation::Landscape).into(),
        layout_items: vec![text_item],
        layout_maps: Vec::new(),
        print_resolution: 300,
        name: "default-layout".to_string(),
    };

    let mut root = ProjectRoot::new_wgs84();
    root.add_layout(layout);

    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let input = &unzip_content(&project.content).expect("failed to unzip xml");

    println!("{input}");
    let expected = include_str!("layout_with_text.xml");

    let skip_config: Vec<SkipNode> = minimal_project_skip_config()
        .into_iter()
        .chain(layout_skip_config())
        .collect();
    xml_comparison(input, expected, Some(&skip_config));
}

#[test]
fn build_minimal_qgs_with_primary_boundaries_xml_works() {
    let layer = generate_pg_map_layer("project_primary_boundaries".to_string());
    let mut root = ProjectRoot::new_wgs84();
    root.add_layer(layer);

    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let additional_skip_nodes = map_layer_skip_config();
    let input = &unzip_content(&project.content).expect("failed to unzip xml");
    println!("{}", input);
    let expected = include_str!("minimal_example_project_with_primary_boundaries.xml");
    let skip_nods: Vec<SkipNode> = minimal_project_skip_config()
        .into_iter()
        .chain(additional_skip_nodes)
        .collect();
    let input = &unzip_content(&project.content).expect("failed to unzip xml");

    xml_comparison(input, expected, Some(&skip_nods));
}

#[test]
fn build_minimal_qgs_with_primary_boundaries_and_layout_xml_works() {
    let layer = generate_pg_map_layer("project_primary_boundaries".to_string());

    let mut map = QgisLayoutMapBuilder::new_bng(uuid::Uuid::new_v4(), "Map 1".into());
    map.srs = None;
    let layout = QgisLayoutBuilder {
        page_size: PageSize::A4(PageOrientation::Landscape).into(),
        layout_maps: vec![map],
        layout_items: Vec::new(),
        print_resolution: 300,
        name: "draft-layout".to_string(),
    };
    let layout_canvas = MapCanvas::for_layout("mAreaCanvas", EPSGID::WGS84);
    let mut root = ProjectRoot::new_wgs84();
    root.add_layer(layer);
    root.add_layout(layout);
    root.map_canvases.push(layout_canvas);

    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let project_skip_nodes = minimal_project_skip_config();
    let layout_skip_nodes = layout_skip_config();
    let map_layer_skip_nodes = map_layer_skip_config();
    let skip_config: Vec<SkipNode> = project_skip_nodes
        .into_iter()
        .chain(layout_skip_nodes)
        .chain(map_layer_skip_nodes)
        .collect();
    let input = &unzip_content(&project.content).expect("failed to unzip xml");
    let expected = include_str!("minimal_example_project_with_primary_boundaries_and_layout.xml");
    xml_comparison(input, expected, Some(&skip_config));
}

#[test]
fn build_layout_with_bng_and_two_layers_works() {
    let mut ds2 = generate_pg_datasource();
    ds2.source = PgSource::PgTable(PgTable {
        schema: "public".into(),
        table_name: "regions_simplified".into(),
    });
    let map = QgisLayoutMapBuilder::new_bng(uuid::Uuid::new_v4(), "Map 1".into());
    let layout_canvas = MapCanvas::for_layout("mAreaCanvas", EPSGID::WGS84);

    let layout = QgisLayoutBuilder {
        page_size: PageSize::A4(PageOrientation::Landscape).into(),
        layout_maps: vec![map],
        layout_items: Vec::new(),
        print_resolution: 300,
        name: "default-layout".to_string(),
    };
    let layer1 = generate_pg_map_layer("project_primary_boundries".to_string());
    let layer2 = QgisMapLayerBuilder {
        layer_name: "regions_simplified".into(),
        legend_text: None,
        include_on_legend: true,
        datasource: DataSource::Postgres(ds2),
        srs: Some(SpatialRefSys::wgs84()),
    }
    .build_vector(WkbType::MultiPolygon);

    let mut root = ProjectRoot::new_wgs84();
    root.add_layout(layout);
    root.add_layer(layer1);
    root.add_layer(layer2);
    root.map_canvases.push(layout_canvas);
    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let project_skip_nodes = minimal_project_skip_config();
    let layout_skip_nodes = layout_skip_config();
    let map_layer_skip_nodes = map_layer_skip_config();
    let mut skip_config: Vec<SkipNode> = project_skip_nodes
        .into_iter()
        .chain(layout_skip_nodes)
        .chain(map_layer_skip_nodes)
        .collect();
    skip_config.push(SkipNode {
        node_name: "datasource".into(),
        attributes_to_skip: vec![],
        children_to_skip: vec![],
        skip_text: true,
    });
    if let Some(node) = skip_config
        .iter_mut()
        .find(|n| n.node_name == "layer-tree-layer")
    {
        node.attributes_to_skip.push("source".into())
    };

    let input = &unzip_content(&project.content).expect("failed to unzip xml");
    let expected = include_str!("layout_with_bng_map_and_two_layers.xml");

    xml_comparison(input, expected, Some(&skip_config));
}

#[test]
fn build_project_with_single_site_boundary_works() {
    let mut ds = generate_pg_datasource();
    ds.source = PgSource::SQL("SELECT * FROM app.site_boundaries WHERE id = 1".into());
    ds.srid = None;
    ds.r#type = None;

    let layer = QgisMapLayerBuilder {
        layer_name: "my-project".into(),
        legend_text: None,
        include_on_legend: true,
        datasource: DataSource::Postgres(ds),
        srs: Some(SpatialRefSys::wgs84()),
    }
    .build_vector(WkbType::MultiPolygon);
    let mut root = ProjectRoot::new_wgs84();
    root.add_layer(layer);
    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let project_skip_nodes = minimal_project_skip_config();
    let map_layer_skip_nodes = map_layer_skip_config();
    let skip_config: Vec<SkipNode> = project_skip_nodes
        .into_iter()
        .chain(map_layer_skip_nodes)
        .collect();
    let input = &unzip_content(&project.content).expect("failed to unzip xml");
    let expected = include_str!("project_with_single_site_boundary.xml");

    xml_comparison(input, expected, Some(&skip_config));
}

#[test]
fn project_with_bng_and_extent_set() {
    let root = ProjectRoot::new(EPSGID::BNG, EPSGID::default_extent(&EPSGID::BNG));
    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");
    let input = &unzip_content(&project.content).expect("failed to unzip xml");

    let expected = include_str!("project-bng.xml");

    let additional_skip_nodes = vec![];

    let skip_nodes: Vec<SkipNode> = minimal_project_skip_config()
        .into_iter()
        .chain(additional_skip_nodes)
        .collect();

    xml_comparison(input, expected, Some(&skip_nodes));
}

#[test]
fn project_with_bng_and_extent_set_with_osm_works() {
    let layer = QgisMapLayerBuilder {
        layer_name: "OpenStreetMap".into(),
        legend_text: None,
        include_on_legend: true,
        datasource: DataSource::XYZ(XYZDataSource {
            url: "https://tile.openstreetmap.org/%7Bz%7D/%7Bx%7D/%7By%7D.png".into(),
        }),
        srs: Some(SpatialRefSys::web_mercator()),
    }
    .build_xyz(false);
    let mut root = ProjectRoot::new_bng();
    root.add_layer(layer);
    let project = QgisProjectBuilder {
        project_name: uuid::Uuid::new_v4().to_string(),
        figure_id: 1,
        low_res: false,
        root,
    }
    .build()
    .expect("failed to build qgis project");

    let input = &unzip_content(&project.content).expect("failed to unzpi xml");
    let expected = include_str!("project-osm.xml");

    let additional_skip_nodes = vec![
        SkipNode {
            node_name: "item".into(),
            attributes_to_skip: vec![],
            children_to_skip: vec![],
            skip_text: true,
        },
        SkipNode {
            node_name: "layer-tree-layer".into(),
            attributes_to_skip: vec!["id".into(), "source".into()],
            children_to_skip: vec![],
            skip_text: false,
        },
        SkipNode {
            node_name: "maplayer".into(),
            attributes_to_skip: vec![],
            children_to_skip: vec![
                "customproperties".into(),
                "elevation".into(),
                "extent".into(),
                "mapTip".into(),
                "pipe-data-defined-properties".into(),
                "wgs84extent".into(),
            ],
            skip_text: false,
        },
        SkipNode {
            node_name: "id".into(),
            attributes_to_skip: vec![],
            children_to_skip: vec![],
            skip_text: true,
        },
        SkipNode {
            node_name: "resourceMetadata".into(),
            children_to_skip: vec![
                "license".into(),
                "rights".into(),
                "abstract".into(),
                "crs".into(),
                "extent".into(),
                "identifier".into(),
                "links".into(),
                "title".into(),
            ],
            attributes_to_skip: vec![],
            skip_text: false,
        },
    ];

    let skip_nodes: Vec<SkipNode> = minimal_project_skip_config()
        .into_iter()
        .chain(additional_skip_nodes)
        .collect();

    xml_comparison(input, expected, Some(&skip_nodes));
}
// #[test]
// fn two_layout_maps_and_4_layers_works() {
//     let map_uuid = uuid::Uuid::new_v4();
//     let mut layout_maps = Vec::new();
//     let srs = EPSGID::BNG;
//     let main_map = QgisLayoutMapBuilder {
//         size: Size {
//             width_mm: 100.,
//             height_mm: 100.,
//         },
//         position: Position { x: 5., y: 5. },
//         extent: srs.default_extent(),
//         srs: Some(srs),
//         map_grid: None,
//         id: "Main Map".into(),
//         uuid: map_uuid,
//         keep_layer_set: true,
//     };
//     let overview_map = QgisLayoutMapBuilder {
//         size: Size {
//             width_mm: 50.,
//             height_mm: 50.,
//         },
//         position: Position { x: 100., y: 100. },
//         extent: srs.default_extent(),
//         srs: Some(srs),
//         map_grid: None,
//         id: "Overview Map".into(),
//         uuid: map_uuid,
//         keep_layer_set: true,
//     };
//     layout_maps.push(main_map);
//     layout_maps.push(overview_map);
//     let layout = QgisLayoutBuilder {
//         page_size: PageSize::A4(PageOrientation::Landscape).into(),
//         layout_maps: layout_maps,
//         layout_items: Vec::new(),
//         print_resolution: 300,
//         name: "defualt-layout".to_string(),
//     };
//     let layer = QgisMapLayerBuilder {
//         layer_name: "openstreetmap".into(),
//         legend_text: None,
//         include_on_legend: true,
//         datasource: DataSource::WMS(WMSDataSource {
//             r#type: crate::layer::WMSDataSourceType::XYZ,
//             url: "https://tile.openstreetmap.org/%7Bz%7D/%7Bx%7D/%7By%7D.png&zmax=19&zmin=0".into(),
//         }),
//         srs: Some(SpatialRefSys::web_mercator()),
//     }
//     .build_wms(false);

//     let mut root = ProjectRoot::new_bng();
//     let project = QgisProjectBuilder {
//         project_name: uuid::Uuid::new_v4().to_string(),
//         figure_id: 1,
//         low_res: false,
//         root,
//     }
//     .build()
//     .expect("failed to build qgis project");

//     let input = &unzip_content(&project.content).expect("failed to unzpi xml");
//     let expected = include_str!("two-layout-maps-4-layers.xml");
//     let additional_skip_nodes: Vec<SkipNode> = vec![];
//     let skip_nodes: Vec<SkipNode> = minimal_project_skip_config()
//         .into_iter()
//         .chain(additional_skip_nodes)
//         .chain(layout_skip_config())
//         .collect();
//     xml_comparison(input, expected, Some(&skip_nodes));
// }
