use crate::qgis::{
    layer::{rendering::LayerRenderingPipeline, *},
    srs::{SpatialRefSys, Srs},
};

use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize, Clone)]
pub struct MapLayer {
    #[serde(rename = "@autoRefreshMode")]
    autoRefreshMode: String,
    #[serde(rename = "@autoRefreshTime")]
    autoRefreshTime: u32,
    #[serde(rename = "@geometry", skip_serializing_if = "Option::is_none")]
    geometry: Option<Geometry>,
    #[serde(rename = "@hasScaleBasedVisibilityFlag")]
    hasScaleBasedVisibilityFlag: u16,
    #[serde(rename = "@labelsEnabled", skip_serializing_if = "Option::is_none")]
    pub labelsEnabled: Option<u8>,
    #[serde(rename = "@legendPlaceholderImage")]
    legendPlaceholderImage: Option<String>,
    #[serde(rename = "@maxScale")]
    maxScale: u32,
    #[serde(rename = "@minScale")]
    minScale: u32,
    #[serde(rename = "@readOnly", skip_serializing_if = "Option::is_none")]
    readOnly: Option<u16>,
    #[serde(rename = "@refreshOnNotifyEnabled")]
    refreshOnNotifyEnabled: u16,
    #[serde(rename = "@refreshOnNotifyMessage")]
    refreshOnNotifyMessage: Option<String>,
    #[serde(rename = "@simplifyAlgorithm", skip_serializing_if = "Option::is_none")]
    simplifyAlgorithm: Option<u16>,
    #[serde(
        rename = "@simplifyDrawingHints",
        skip_serializing_if = "Option::is_none"
    )]
    simplifyDrawingHints: Option<u16>,
    #[serde(
        rename = "@simplifyDrawingTol",
        skip_serializing_if = "Option::is_none"
    )]
    simplifyDrawingTol: Option<u16>,
    #[serde(rename = "@simplifyMaxScale", skip_serializing_if = "Option::is_none")]
    simplifyMaxScale: Option<u16>,
    #[serde(rename = "@simplifyLocal", skip_serializing_if = "Option::is_none")]
    simplifyLocal: Option<u16>,
    #[serde(rename = "@styleCategories")]
    styleCategories: String,
    #[serde(
        rename = "@symbologyReferenceScale",
        skip_serializing_if = "Option::is_none"
    )]
    symbologyReferenceScale: Option<i16>,
    #[serde(rename = "@type")]
    r#type: DataType,
    #[serde(rename = "@wkbType", skip_serializing_if = "Option::is_none")]
    wkbType: Option<WkbType>,
    pub id: String,
    pub datasource: DataSource,
    keywordList: KeywordList,
    pub layername: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    srs: Option<Srs>,
    resourceMetadata: ResourceMetadata,
    pub provider: Provider,
    #[serde(skip_serializing_if = "Option::is_none")]
    vectorjoins: Option<VectorJoins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layerDependencies: Option<LayerDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataDependencies: Option<DataDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expressionfields: Option<Vec<ExpressionFields>>,
    #[serde(rename = "map-layer-style-manager")]
    mapLayerStyleManager: MapLayerStyleManager,
    #[serde(skip_serializing_if = "Option::is_none")]
    auxiliaryLayer: Option<AuxiliaryLayer>,
    metadataUrls: MetadataUrls,
    flags: Flags,
    #[serde(skip_serializing_if = "Option::is_none")]
    blendMode: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noData: Option<NoData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporal: Option<Temporal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipe: Option<LayerRenderingPipeline>,
    #[serde(rename = "renderer-v2", skip_serializing_if = "Option::is_none")]
    renderer_v2: Option<RendererV2>,
    #[serde(skip_serializing)]
    pub legend_text: Option<String>,
    #[serde(skip_serializing)]
    pub include_on_legend: bool,
}

#[derive(Serialize, Clone)]
struct RendererV2 {
    #[serde(rename = "@symbollevels")]
    symbollevels: u16,
    #[serde(rename = "@forceraster")]
    forceraster: u16,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@enableorderby")]
    enableorderby: u16,
    #[serde(rename = "@referencescale")]
    referencescale: i16,
    #[serde(rename = "$text")]
    text: String,
}
impl Default for MapLayer {
    fn default() -> Self {
        Self {
            autoRefreshMode: "Disabled".into(),
            autoRefreshTime: Default::default(),
            geometry: Default::default(),
            hasScaleBasedVisibilityFlag: Default::default(),
            labelsEnabled: Default::default(),
            legendPlaceholderImage: Default::default(),
            maxScale: Default::default(),
            minScale: 100000000,
            readOnly: Default::default(),
            refreshOnNotifyEnabled: Default::default(),
            refreshOnNotifyMessage: Default::default(),
            simplifyAlgorithm: Default::default(),
            simplifyDrawingHints: None,
            simplifyDrawingTol: None,
            simplifyMaxScale: None,
            simplifyLocal: None,
            styleCategories: "AllStyleCategories".into(),
            symbologyReferenceScale: None,
            r#type: DataType::raster,
            wkbType: Default::default(),
            id: Default::default(),
            datasource: Default::default(),
            keywordList: Default::default(),
            layername: Default::default(),
            srs: None,
            resourceMetadata: Default::default(),
            provider: Default::default(),
            vectorjoins: Default::default(),
            layerDependencies: Default::default(),
            dataDependencies: Default::default(),
            expressionfields: Default::default(),
            mapLayerStyleManager: Default::default(),
            auxiliaryLayer: Default::default(),
            metadataUrls: Default::default(),
            flags: Default::default(),
            legend_text: Default::default(),
            include_on_legend: Default::default(),
            blendMode: Default::default(),
            noData: Default::default(),
            temporal: Default::default(),
            pipe: Default::default(),
            renderer_v2: Default::default(),
        }
    }
}

pub struct QgisMapLayerBuilder {
    pub layer_name: String,
    pub legend_text: Option<String>,
    pub include_on_legend: bool,
    pub datasource: DataSource,
    pub srs: Option<SpatialRefSys>,
}

impl QgisMapLayerBuilder {
    fn generate_id(&self) -> String {
        format!(
            "{}_{}",
            slug::slugify(&self.layer_name),
            uuid::Uuid::new_v4()
        )
    }
    fn build(self) -> MapLayer {
        MapLayer {
            id: self.generate_id(),
            layername: self.layer_name,
            legend_text: self.legend_text,
            include_on_legend: self.include_on_legend,
            datasource: self.datasource,
            srs: self.srs.map(|s| Srs { spatialrefsys: s }),
            ..Default::default()
        }
    }

    pub fn build_raster(self, greyscale: bool) -> Result<MapLayer, anyhow::Error> {
        match self.datasource {
            DataSource::Postgres(_) => Err(anyhow::anyhow!(
                "cannot build as raster as datasource is vector datasource."
            )),
            DataSource::WMS(_) => Ok(self.build_wms(greyscale)),
            DataSource::XYZ(_) => Ok(self.build_xyz(greyscale)),
        }
    }

    pub fn build_vector(self, wkb_type: WkbType) -> MapLayer {
        let mut layer = self.build();
        layer.geometry = Some((&wkb_type).into());
        layer.wkbType = Some(wkb_type);
        layer.labelsEnabled = Some(0);
        layer.readOnly = Some(0);
        layer.simplifyAlgorithm = Some(0);
        layer.simplifyDrawingHints = Some(1);
        layer.simplifyDrawingTol = Some(1);
        layer.simplifyLocal = Some(1);
        layer.simplifyMaxScale = Some(1);
        layer.symbologyReferenceScale = Some(-1);
        layer.r#type = DataType::vector;
        layer.auxiliaryLayer = Some(AuxiliaryLayer::default());
        layer.dataDependencies = Some(DataDependencies::default());
        layer.expressionfields = Some(vec![
            ExpressionFields::default(),
            ExpressionFields::default(),
        ]);
        layer.layerDependencies = Some(LayerDependencies::default());
        layer.provider.encoding = Some("".into());
        layer.vectorjoins = Some(VectorJoins {});

        layer
    }
    pub fn build_xyz(self, greyscale: bool) -> MapLayer {
        let mut layer = self.build();
        layer.r#type = DataType::raster;
        layer.blendMode = Some(0);
        layer.noData = Some(Default::default());
        layer.provider.text = DataProvider::wms;
        layer.temporal = Some(Default::default());
        layer.flags.searchable = 0;
        let mut layer_rendering_pipeline = LayerRenderingPipeline::default();
        if greyscale {
            layer_rendering_pipeline.huesaturation.greyscaleMode = 2
        }
        layer.pipe = Some(layer_rendering_pipeline);

        layer
    }

    pub fn build_wms(self, greyscale: bool) -> MapLayer {
        let mut layer = Self::build_xyz(self, greyscale);
        layer.flags.searchable = 1;

        layer
    }
}

#[cfg(test)]
mod tests {

    use serde::Serialize;

    use crate::qgis::{
        helpers::{extract_renderer_v2, insert_renderer_v2_into_project},
        layer::{
            DataSource, MapLayer, QgisMapLayerBuilder, WkbType,
            datasource::{wms::WMSDataSource, xyz::XYZDataSource},
        },
        srs::SpatialRefSys,
        tests::test_helpers::{SkipNode, generate_pg_datasource, xml_comparison},
    };

    #[test]
    fn wms_layer_works() {
        let layer = QgisMapLayerBuilder {
            layer_name: "OpenStreetMap".into(),
            legend_text: None,
            include_on_legend: false,
            datasource: DataSource::XYZ(XYZDataSource {
                url: r"https://tile.openstreetmap.org/{z}/{x}/{y}.png".into(),
            }),
            srs: None,
        }
        .build_xyz(false);
        let skip_config = vec![
            SkipNode {
                node_name: "MapLayer".into(),
                attributes_to_skip: vec![],
                children_to_skip: vec![
                    "customproperties".into(),
                    "elevation".into(),
                    "extent".into(),
                    "mapTip".into(),
                    "pipe-data-defined-properties".into(),
                    "wgs84extent".into(),
                    "srs".into(),
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
                    "contact".into(),
                ],
                attributes_to_skip: vec![],
                skip_text: false,
            },
            SkipNode {
                node_name: "spatialrefsys".into(),
                attributes_to_skip: vec!["nativeFormat".into()],
                children_to_skip: vec!["wkt".into()],
                skip_text: false,
            },
        ];

        let xml = quick_xml::se::to_string(&layer).expect("failed to convert layer to xml string");
        let expected = include_str!("examples/open-street-map.xml");
        xml_comparison(&xml, expected, Some(&skip_config));
        let layer = QgisMapLayerBuilder {
            layer_name: "OpenStreetMap".into(),
            legend_text: None,
            include_on_legend: false,
            datasource: DataSource::XYZ(XYZDataSource {
                url: r"https://tile.openstreetmap.org/{z}/{x}/{y}.png".into(),
            }),
            srs: None,
        }
        .build_xyz(true);
        let xml = quick_xml::se::to_string(&layer).expect("failed to convert layer to xml string");
        let greyscale_expected = include_str!("examples/greyscale-osm.xml");
        xml_comparison(&xml, greyscale_expected, Some(&skip_config));
    }

    #[test]
    fn os_wms_works() {
        let layer = QgisMapLayerBuilder {
            layer_name: "OS 50k".into(),
            legend_text: None,
            include_on_legend: false,
            datasource: DataSource::WMS(WMSDataSource::new_wms(
                Some("y1mj99p".to_string()),
                "https://geoserver.geodata-manager.com/geoserver/wms".to_string(),
                "osdata:50k".to_string(),
                27700,
            )),
            srs: None,
        }
        .build_wms(false);
        let xml = quick_xml::se::to_string(&layer).expect("failed to convert layer to xml string");
        let expected = include_str!("examples/os50k_wms.xml");
        let skip_config = vec![
            SkipNode {
                node_name: "MapLayer".into(),
                attributes_to_skip: vec![],
                children_to_skip: vec![
                    "customproperties".into(),
                    "elevation".into(),
                    "extent".into(),
                    "mapTip".into(),
                    "pipe-data-defined-properties".into(),
                    "wgs84extent".into(),
                    "srs".into(),
                    "blendMode".into(),
                    "noData".into(),
                    "pipe".into(),
                    "temporal".into(),
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
                    "contact".into(),
                    "type".into(),
                ],
                attributes_to_skip: vec![],
                skip_text: false,
            },
            SkipNode {
                node_name: "spatialrefsys".into(),
                attributes_to_skip: vec!["nativeFormat".into()],
                children_to_skip: vec!["wkt".into()],
                skip_text: false,
            },
        ];
        xml_comparison(&xml, expected, Some(&skip_config));
        let layer = QgisMapLayerBuilder {
            layer_name: "OS 50k".into(),
            legend_text: None,
            include_on_legend: false,
            datasource: DataSource::WMS(WMSDataSource::new_wmts(
                Some("y1mj99p".to_string()),
                "https://geoserver.geodata-manager.com/geoserver/gwc/service/wmts".to_string(),
                "osdata:50k".to_string(),
                27700,
                "50kMapTiles".into(),
            )),
            srs: None,
        }
        .build_wms(false);
        let xml = quick_xml::se::to_string(&layer).expect("failed to convert layer to xml string");
        let expected = include_str!("examples/os50k_wmts.xml");
        xml_comparison(&xml, expected, Some(&skip_config));
    }

    #[derive(Serialize)]
    struct Root {
        maplayer: MapLayer,
    }
    #[test]
    fn primary_boundaries_with_style_works() {
        let layer_name = "project_primary_boundaries";
        let maplayer = QgisMapLayerBuilder {
            layer_name: layer_name.into(),
            legend_text: None,
            include_on_legend: true,
            datasource: DataSource::Postgres(generate_pg_datasource()),
            srs: Some(SpatialRefSys::wgs84()),
        }
        .build_vector(WkbType::MultiPolygon);
        let root = Root { maplayer };

        let xml = quick_xml::se::to_string(&root).expect("failed to seriazlize to xml");
        let expected = include_str!("examples/primary_boundaries.xml");
        let style = extract_renderer_v2(include_str!("examples/style_file.xml"))
            .expect("failed to extract rendere v2");
        let input = insert_renderer_v2_into_project(&xml, &style, layer_name.into())
            .expect("failed to insert style");

        let skip_nodes = vec![
            SkipNode {
                node_name: "maplayer".into(),
                children_to_skip: vec![
                    "aliases".into(),
                    "attributeactions".into(),
                    "attributetableconfig".into(),
                    "temporal".into(),
                    "elevation".into(),
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
                    "srs".into(),
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
        ];

        xml_comparison(&input, expected, Some(&skip_nodes));
    }
}
