use serde::Serialize;

use crate::{
    LayerTreeGroup, QgisUuid,
    clipping_settings::ClippingSettings,
    enums::{HorizontalAlignment, LineJoinStyle, ReferencePoint, VerticalAlignment},
    extent::LayoutMapExtent,
    layout::components::{
        Color, LayoutObject, Length, Nodes, Position, Size, TextStyle, layout_item::components::*,
    },
    srs::Srs,
    symbol::Symbol,
};

#[derive(Serialize, Default)]
#[allow(non_snake_case)]
pub struct LayoutItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    ComposerMapOverview: Option<ComposerMapOverview>,
    #[serde(rename = "@opacity")]
    opacity: f32,
    #[serde(rename = "@templateUuid")]
    template_uuid: String,
    #[serde(rename = "@drawCanvasItems", skip_serializing_if = "Option::is_none")]
    draw_canvas_items: Option<bool>,
    #[serde(rename = "@frameJoinStyle")]
    frame_join_style: LineJoinStyle,
    #[serde(rename = "@groupUuid")]
    group_uuid: String,
    #[serde(rename = "@isTemporal", skip_serializing_if = "Option::is_none")]
    is_temporal: Option<u32>,
    #[serde(rename = "@visibility")]
    visibility: u16,
    #[serde(rename = "@positionOnPage")]
    position_on_page: Position,
    #[serde(rename = "@size")]
    pub size: Size,
    #[serde(rename = "@excludeFromExports")]
    exclude_from_exports: u16,
    #[serde(rename = "@type")]
    item_type: u32,
    #[serde(rename = "@zValue")]
    pub z_value: u32,
    #[serde(rename = "@labelMargin", skip_serializing_if = "Option::is_none")]
    label_margin: Option<Length>,
    #[serde(rename = "@referencePoint")]
    pub reference_point: ReferencePoint,
    #[serde(rename = "@keepLayerSet", skip_serializing_if = "Option::is_none")]
    pub keep_layer_set: Option<bool>,
    #[serde(rename = "@followPreset", skip_serializing_if = "Option::is_none")]
    follow_preset: Option<bool>,
    #[serde(rename = "@mapFlags", skip_serializing_if = "Option::is_none")]
    map_flags: Option<u32>,
    #[serde(rename = "@positionLock")]
    position_lock: bool,
    #[serde(rename = "@mapRotation", skip_serializing_if = "Option::is_none")]
    map_rotation: Option<u32>,
    #[serde(rename = "@followPresetName", skip_serializing_if = "Option::is_none")]
    follow_preset_name: Option<String>,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@background")]
    pub background: bool,
    #[serde(rename = "@outlineWidthM")]
    outline_width_m: Length,
    #[serde(rename = "@itemRotation")]
    item_rotation: u32,
    #[serde(rename = "@position")]
    position: Position,
    #[serde(rename = "@frame")]
    pub frame: bool,
    #[serde(rename = "@blendMode")]
    blend_mode: u16,
    #[serde(rename = "@uuid")]
    uuid: QgisUuid,
    #[serde(rename = "FrameColor")]
    frame_color: Color,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Color,
    #[serde(rename = "LayoutObject")]
    layout_object: LayoutObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "AtlasMap", skip_serializing_if = "Option::is_none")]
    atlas_map: Option<components::AtlasMap>,
    #[serde(rename = "Extent", skip_serializing_if = "Option::is_none")]
    extent: Option<LayoutMapExtent>,
    #[serde(rename = "LayerSet", skip_serializing_if = "Option::is_none")]
    layer_set: Option<LayerSet>,
    #[serde(
        rename = "atlasClippingSettings",
        skip_serializing_if = "Option::is_none"
    )]
    atlas_clipping_settings: Option<ClippingSettings>,
    #[serde(
        rename = "itemClippingSettings",
        skip_serializing_if = "Option::is_none"
    )]
    item_clipping_settings: Option<ClippingSettings>,
    #[serde(rename = "labelBlockingItems", skip_serializing_if = "Option::is_none")]
    label_blocking_items: Option<LabelBlockingItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crs: Option<Srs>,
    #[serde(rename = "ComposerMapGrid", skip_serializing_if = "Option::is_none")]
    pub composer_map_grid: Option<ComposerMapGrid>,
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<HorizontalAlignment>,
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<VerticalAlignment>,
    #[serde(rename = "@htmlState", skip_serializing_if = "Option::is_none")]
    pub htmlState: Option<u8>,
    #[serde(rename = "@labelText", skip_serializing_if = "Option::is_none")]
    labelText: Option<String>,
    #[serde(rename = "@marginX", skip_serializing_if = "Option::is_none")]
    pub marginX: Option<f32>,
    #[serde(rename = "@marginY", skip_serializing_if = "Option::is_none")]
    pub marginY: Option<f32>,
    #[serde(rename = "text-style", skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    #[serde(
        rename = "@arrowHeadFillColor",
        skip_serializing_if = "Option::is_none"
    )]
    arrowHeadFillColor: Option<String>,
    #[serde(
        rename = "@arrowHeadOutlineColor",
        skip_serializing_if = "Option::is_none"
    )]
    arrowHeadOutlineColor: Option<String>,
    #[serde(rename = "@arrowHeadWidth", skip_serializing_if = "Option::is_none")]
    arrowHeadWidth: Option<f32>,
    #[serde(rename = "@markerMode", skip_serializing_if = "Option::is_none")]
    markerMode: Option<u16>,
    #[serde(rename = "@outlineWidth", skip_serializing_if = "Option::is_none")]
    outlineWidth: Option<f32>,
    #[serde(rename = "@startMarkerMode", skip_serializing_if = "Option::is_none")]
    startMarkerMode: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Nodes>,
    #[serde(rename = "@alignment", skip_serializing_if = "Option::is_none")]
    alignment: Option<i32>,
    #[serde(rename = "@boxContentSpace", skip_serializing_if = "Option::is_none")]
    boxContentSpace: Option<i32>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    height: Option<f32>,
    #[serde(
        rename = "@labelHorizontalPlacement",
        skip_serializing_if = "Option::is_none"
    )]
    labelHorizontalPlacement: Option<f32>,
    // Scalebar-specific attributes
    #[serde(rename = "@lineCapStyle", skip_serializing_if = "Option::is_none")]
    line_cap_style: Option<String>,
    #[serde(
        rename = "@subdivisionsHeight",
        skip_serializing_if = "Option::is_none"
    )]
    subdivisions_height: Option<f32>,
    #[serde(rename = "@minBarWidth", skip_serializing_if = "Option::is_none")]
    min_bar_width: Option<u32>,
    #[serde(
        rename = "@labelVerticalPlacement",
        skip_serializing_if = "Option::is_none"
    )]
    label_vertical_placement: Option<u32>,
    #[serde(rename = "@maxBarWidth", skip_serializing_if = "Option::is_none")]
    max_bar_width: Option<u32>,
    #[serde(rename = "@unitLabel", skip_serializing_if = "Option::is_none")]
    unit_label: Option<String>,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(rename = "@numSegments", skip_serializing_if = "Option::is_none")]
    num_segments: Option<u32>,
    #[serde(rename = "@lineJoinStyle", skip_serializing_if = "Option::is_none")]
    line_join_style: Option<String>,
    #[serde(
        rename = "@numUnitsPerSegment",
        skip_serializing_if = "Option::is_none"
    )]
    num_units_per_segment: Option<u32>,
    #[serde(
        rename = "@numMapUnitsPerScaleBarUnit",
        skip_serializing_if = "Option::is_none"
    )]
    num_map_units_per_scale_bar_unit: Option<u32>,
    #[serde(rename = "@numSubdivisions", skip_serializing_if = "Option::is_none")]
    num_subdivisions: Option<u32>,
    #[serde(
        rename = "@segmentMillimeters",
        skip_serializing_if = "Option::is_none"
    )]
    segment_millimeters: Option<u32>,
    #[serde(rename = "@mapUuid", skip_serializing_if = "Option::is_none")]
    map_uuid: Option<QgisUuid>,
    #[serde(rename = "@map_uuid", skip_serializing_if = "Option::is_none")]
    legend_map_uuid: Option<QgisUuid>,
    #[serde(rename = "@segmentSizeMode", skip_serializing_if = "Option::is_none")]
    segment_size_mode: Option<u32>,
    #[serde(rename = "@labelBarSpace", skip_serializing_if = "Option::is_none")]
    label_bar_space: Option<u32>,
    #[serde(rename = "@numSegmentsLeft", skip_serializing_if = "Option::is_none")]
    num_segments_left: Option<u32>,
    #[serde(rename = "@unitType", skip_serializing_if = "Option::is_none")]
    unit_type: Option<String>,
    // Image-specific attributes
    #[serde(rename = "@anchorPoint", skip_serializing_if = "Option::is_none")]
    pub anchor_point: Option<ReferencePoint>,
    #[serde(rename = "@svgFillColor", skip_serializing_if = "Option::is_none")]
    svg_fill_color: Option<String>,
    #[serde(rename = "@northOffset", skip_serializing_if = "Option::is_none")]
    north_offset: Option<f64>,
    #[serde(rename = "@pictureRotation", skip_serializing_if = "Option::is_none")]
    picture_rotation: Option<f64>,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    mode: Option<u32>,
    #[serde(rename = "@northMode", skip_serializing_if = "Option::is_none")]
    north_mode: Option<u32>,
    #[serde(rename = "@svgBorderWidth", skip_serializing_if = "Option::is_none")]
    svg_border_width: Option<f64>,
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(rename = "@svgBorderColor", skip_serializing_if = "Option::is_none")]
    svg_border_color: Option<String>,
    #[serde(rename = "@resizeMode", skip_serializing_if = "Option::is_none")]
    resize_mode: Option<u32>,
    #[serde(rename = "@pictureWidth", skip_serializing_if = "Option::is_none")]
    picture_width: Option<f64>,
    #[serde(rename = "@pictureHeight", skip_serializing_if = "Option::is_none")]
    picture_height: Option<f64>,
    // Legend-specific attributes
    #[serde(rename = "@titleAlignment", skip_serializing_if = "Option::is_none")]
    title_alignment: Option<u32>,
    #[serde(rename = "@boxSpace", skip_serializing_if = "Option::is_none")]
    box_space: Option<u32>,
    #[serde(rename = "@equalColumnWidth", skip_serializing_if = "Option::is_none")]
    equal_column_width: Option<u32>,
    #[serde(rename = "@maxSymbolSize", skip_serializing_if = "Option::is_none")]
    max_symbol_size: Option<u32>,
    #[serde(rename = "@wmsLegendWidth", skip_serializing_if = "Option::is_none")]
    wms_legend_width: Option<u32>,
    #[serde(rename = "@resizeToContents", skip_serializing_if = "Option::is_none")]
    resize_to_contents: Option<u8>,
    #[serde(rename = "@wrapChar", skip_serializing_if = "Option::is_none")]
    pub wrap_char: Option<String>,
    #[serde(rename = "@symbolWidth", skip_serializing_if = "Option::is_none")]
    symbol_width: Option<u32>,
    #[serde(rename = "@rasterBorder", skip_serializing_if = "Option::is_none")]
    raster_border: Option<u32>,
    #[serde(rename = "@columnCount", skip_serializing_if = "Option::is_none")]
    column_count: Option<u32>,
    #[serde(rename = "@symbolHeight", skip_serializing_if = "Option::is_none")]
    symbol_height: Option<u32>,
    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "@splitLayer", skip_serializing_if = "Option::is_none")]
    split_layer: Option<u32>,
    #[serde(rename = "@minSymbolSize", skip_serializing_if = "Option::is_none")]
    min_symbol_size: Option<u32>,
    #[serde(rename = "@rasterBorderWidth", skip_serializing_if = "Option::is_none")]
    raster_border_width: Option<u32>,
    #[serde(rename = "@rasterBorderColor", skip_serializing_if = "Option::is_none")]
    raster_border_color: Option<String>,
    #[serde(rename = "@columnSpace", skip_serializing_if = "Option::is_none")]
    column_space: Option<u32>,
    #[serde(rename = "@symbolAlignment", skip_serializing_if = "Option::is_none")]
    symbol_alignment: Option<u32>,
    #[serde(rename = "@wmsLegendHeight", skip_serializing_if = "Option::is_none")]
    wms_legend_height: Option<u32>,
    #[serde(
        rename = "@legendFilterByAtlas",
        skip_serializing_if = "Option::is_none"
    )]
    legend_filter_by_atlas: Option<u32>,
    // Scalebar styling elements
    #[serde(rename = "strokeColor", skip_serializing_if = "Option::is_none")]
    stroke_color: Option<Color>,
    #[serde(rename = "numericFormat", skip_serializing_if = "Option::is_none")]
    numeric_format: Option<NumericFormat>,
    #[serde(rename = "fillColor", skip_serializing_if = "Option::is_none")]
    fill_color: Option<Color>,
    #[serde(rename = "fillColor2", skip_serializing_if = "Option::is_none")]
    fill_color2: Option<Color>,
    #[serde(rename = "lineSymbol", skip_serializing_if = "Option::is_none")]
    line_symbol: Option<LayoutItemSymbol>,
    #[serde(rename = "divisionLineSymbol", skip_serializing_if = "Option::is_none")]
    division_line_symbol: Option<LayoutItemSymbol>,
    #[serde(
        rename = "subdivisionLineSymbol",
        skip_serializing_if = "Option::is_none"
    )]
    subdivision_line_symbol: Option<LayoutItemSymbol>,
    #[serde(rename = "fillSymbol1", skip_serializing_if = "Option::is_none")]
    fill_symbol1: Option<LayoutItemSymbol>,
    #[serde(rename = "fillSymbol2", skip_serializing_if = "Option::is_none")]
    fill_symbol2: Option<LayoutItemSymbol>,
    // Legend styles
    #[serde(rename = "styles", skip_serializing_if = "Option::is_none")]
    styles: Option<legend::LegendStyles>,
    #[serde(rename = "layer-tree-group", skip_serializing_if = "Option::is_none")]
    layer_tree_group: Option<LayerTreeGroup>,
}

impl LayoutItem {
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
        self.position_on_page = position;
    }
    pub fn set_z_value(&mut self, z_value: u32) {
        self.z_value = z_value
    }
}

mod components;
mod image;
mod legend;
mod map;
mod north_arrow;
mod page;
mod polyline;
mod scalebar;
mod text;
pub use components::ComposerMapGrid;
pub use legend::LegendTextStyles;
mod polygon;
