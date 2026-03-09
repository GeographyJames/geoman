pub mod components;

pub use components::Size;

mod layout_map_builder;
pub use layout_map_builder::QgisLayoutMapBuilder;
mod page_size;
pub use page_size::{PageOrientation, PageSize};

use serde::Serialize;

use crate::qgis::{Symbol, layout::components::LayoutItem};

pub struct QgisLayoutBuilder {
    pub page_size: Size,
    pub layout_maps: Vec<QgisLayoutMapBuilder>,
    pub layout_items: Vec<LayoutItem>,
    pub print_resolution: u32,
    pub name: String,
}

impl QgisLayoutBuilder {
    pub fn build(self) -> QgisLayout {
        let mut layout = QgisLayout {
            print_resolution: self.print_resolution,
            name: self.name,
            ..QgisLayout::default()
        };
        layout.page_collection = PageCollection::new(self.page_size);
        layout.layout_items = Some(
            self.layout_maps
                .into_iter()
                .map(|m| m.build())
                .chain(self.layout_items)
                .collect(),
        );
        layout
    }
}

#[derive(Serialize)]
#[serde(rename = "Layout")]
pub struct QgisLayout {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@printResolution")]
    print_resolution: u32,
    #[serde(rename = "@units")]
    units: String,
    #[serde(rename = "Grid")]
    grid: Grid,
    #[serde(rename = "PageCollection")]
    page_collection: PageCollection,
    #[serde(rename = "Atlas")]
    atlas: Atlas,
    #[serde(rename = "LayoutItem", skip_serializing_if = "Option::is_none")]
    layout_items: Option<Vec<LayoutItem>>,
}

#[derive(Serialize)]
struct Grid {
    #[serde(rename = "@offsetX")]
    offset_x: String,
    #[serde(rename = "@offsetUnits")]
    offset_units: String,
    #[serde(rename = "@offsetY")]
    offset_y: String,
    #[serde(rename = "@resUnits")]
    res_units: String,
    #[serde(rename = "@resolution")]
    resolution: String,
}

#[derive(Serialize)]
struct PageCollection {
    symbol: Symbol,
    #[serde(rename = "LayoutItem")]
    layout_items: Vec<LayoutItem>,
    #[serde(rename = "GuideCollection")]
    guide_collection: GuideCollection,
}

#[derive(Serialize)]
struct GuideCollection {
    #[serde(rename = "@visible")]
    visible: String,
}

#[derive(Serialize)]
struct Atlas {
    #[serde(rename = "@hideCoverage")]
    hide_coverage: String,
    #[serde(rename = "@pageNameExpression")]
    page_name_expression: String,
    #[serde(rename = "@filterFeatures")]
    filter_features: String,
    #[serde(rename = "@enabled")]
    enabled: String,
    #[serde(rename = "@coverageLayer")]
    coverage_layer: String,
    #[serde(rename = "@filenamePattern")]
    filename_pattern: String,
    #[serde(rename = "@sortFeatures")]
    sort_features: String,
}

impl Default for QgisLayout {
    fn default() -> Self {
        Self {
            name: "default-layout".to_string(),
            print_resolution: 300,
            units: "mm".to_string(),
            grid: Grid::default(),
            page_collection: PageCollection::default(),
            atlas: Atlas::default(),
            layout_items: Default::default(),
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            offset_x: "0".to_string(),
            offset_units: "mm".to_string(),
            offset_y: "0".to_string(),
            res_units: "mm".to_string(),
            resolution: "10".to_string(),
        }
    }
}

impl Default for PageCollection {
    fn default() -> Self {
        Self {
            symbol: Symbol::default(),
            layout_items: vec![LayoutItem::page(
                PageSize::A4(PageOrientation::Landscape).into(),
            )],
            guide_collection: GuideCollection::default(),
        }
    }
}

impl PageCollection {
    fn new(page_size: Size) -> Self {
        Self {
            symbol: Symbol::default(),
            layout_items: vec![LayoutItem::page(page_size)],
            guide_collection: GuideCollection::default(),
        }
    }
}

impl Default for GuideCollection {
    fn default() -> Self {
        Self {
            visible: "1".to_string(),
        }
    }
}

impl Default for Atlas {
    fn default() -> Self {
        Self {
            hide_coverage: "0".to_string(),
            page_name_expression: "".to_string(),
            filter_features: "0".to_string(),
            enabled: "0".to_string(),
            coverage_layer: "".to_string(),
            filename_pattern: "'output_'||@atlas_featurenumber".to_string(),
            sort_features: "0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::qgis::tests::test_helpers::xml_comparison;

    use super::*;

    #[test]
    fn layout_works() {
        let expected = r#"<Layout name="default-layout" printResolution="300" units="mm"><Grid offsetX="0" offsetUnits="mm" offsetY="0" resUnits="mm" resolution="10"/><PageCollection><symbol clip_to_extent="1" type="fill" name="" force_rhr="0" is_animated="0" alpha="1" frame_rate="10"><data_defined_properties><Option type="Map"><Option value="" type="QString" name="name"/><Option value="collection" type="QString" name="type"/></Option></data_defined_properties><layer class="SimpleFill" enabled="1" locked="0" id="{b9ee01da-b261-4282-8755-8d7490dae48c}" pass="0"><Option type="Map"><Option value="3x:0,0,0,0,0,0" type="QString" name="border_width_map_unit_scale"/><Option value="no" type="QString" name="outline_style"/><Option value="255,255,255,255" type="QString" name="color"/><Option value="solid" type="QString" name="style"/></Option><data_defined_properties><Option type="Map"><Option value="" type="QString" name="name"/><Option value="collection" type="QString" name="type"/></Option></data_defined_properties></layer></symbol><LayoutItem opacity="1" templateUuid="" frameJoinStyle="miter" groupUuid="" visibility="1" positionOnPage="0,0,mm" size="297,210,mm" excludeFromExports="0" type="65638" zValue="0" referencePoint="0" positionLock="false" id="" background="true" outlineWidthM="0.3,mm" itemRotation="0" position="0,0,mm" frame="false" blendMode="0" uuid="{78b0a83c-2993-43ba-b319-81394c21c1c5}"><FrameColor red="0" blue="0" alpha="255" green="0"/><BackgroundColor red="255" blue="255" alpha="255" green="255"/><LayoutObject><dataDefinedProperties><Option type="Map"><Option value="" type="QString" name="name"/><Option value="collection" type="QString" name="type"/></Option></dataDefinedProperties></LayoutObject><symbol clip_to_extent="1" type="fill" name="" force_rhr="0" is_animated="0" alpha="1" frame_rate="10"><data_defined_properties><Option type="Map"><Option value="" type="QString" name="name"/><Option value="collection" type="QString" name="type"/></Option></data_defined_properties><layer class="SimpleFill" enabled="1" locked="0" id="{fc59f952-ac28-40ee-b678-b50c90510f77}" pass="0"><Option type="Map"><Option value="3x:0,0,0,0,0,0" type="QString" name="border_width_map_unit_scale"/><Option value="no" type="QString" name="outline_style"/><Option value="255,255,255,255" type="QString" name="color"/><Option value="solid" type="QString" name="style"/></Option><data_defined_properties><Option type="Map"><Option value="" type="QString" name="name"/><Option value="collection" type="QString" name="type"/></Option></data_defined_properties></layer></symbol></LayoutItem><GuideCollection visible="1"/></PageCollection><Atlas hideCoverage="0" pageNameExpression="" filterFeatures="0" enabled="0" coverageLayer="" filenamePattern="'output_'||@atlas_featurenumber" sortFeatures="0"/></Layout>"#;

        xml_comparison(
            &quick_xml::se::to_string(&QgisLayout::default()).expect("failed to serialize to xml"),
            expected,
            None,
        );
    }
}
