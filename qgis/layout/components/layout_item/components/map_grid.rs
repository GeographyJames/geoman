use crate::qgis::{
    QgisUuid,
    data_defined_properties::DataDefinedProperties,
    enums::{LineCapStyle, LineJoinStyle},
    layout::components::{Color, LayoutObject, TextStyle, layout_object::CustomProperties},
    symbol::Symbol,
};
use serde::Serialize;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ComposerMapGrid {
    #[serde(rename = "@gridFrameMargin")]
    grid_frame_margin: f64,
    #[serde(rename = "@leftAnnotationPosition")]
    left_annotation_position: u32,
    #[serde(rename = "@rotatedTicksMarginToCorner")]
    rotated_ticks_margin_to_corner: f64,
    #[serde(rename = "@intervalX")]
    interval_x: f64,
    #[serde(rename = "@leftAnnotationDisplay")]
    left_annotation_display: u32,
    #[serde(rename = "@offsetX")]
    offset_x: f64,
    #[serde(rename = "@showAnnotation")]
    show_annotation: u32,
    #[serde(rename = "@annotationPrecision")]
    annotation_precision: u32,
    #[serde(rename = "@rotatedAnnotationsEnabled")]
    rotated_annotations_enabled: u32,
    #[serde(rename = "@rightAnnotationDisplay")]
    right_annotation_display: u32,
    #[serde(rename = "@gridFrameWidth")]
    grid_frame_width: f64,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@intervalY")]
    interval_y: f64,
    #[serde(rename = "@rotatedAnnotationsMinimumAngle")]
    rotated_annotations_minimum_angle: f64,
    #[serde(rename = "@unit")]
    unit: u32,
    #[serde(rename = "@bottomAnnotationDirection")]
    bottom_annotation_direction: u32,
    #[serde(rename = "@minimumIntervalWidth")]
    minimum_interval_width: f64,
    #[serde(rename = "@annotationFormat")]
    annotation_format: u32,
    #[serde(rename = "@rotatedAnnotationsMarginToCorner")]
    rotated_annotations_margin_to_corner: f64,
    #[serde(rename = "@offsetY")]
    offset_y: f64,
    #[serde(rename = "@gridFramePenThickness")]
    grid_frame_pen_thickness: f64,
    #[serde(rename = "@annotationExpression")]
    annotation_expression: String,
    #[serde(rename = "@frameFillColor1")]
    frame_fill_color1: String,
    #[serde(rename = "@maximumIntervalWidth")]
    maximum_interval_width: f64,
    #[serde(rename = "@rotatedAnnotationsLengthMode")]
    rotated_annotations_length_mode: u32,
    #[serde(rename = "@frameFillColor2")]
    frame_fill_color2: String,
    #[serde(rename = "@gridFrameStyle")]
    grid_frame_style: u32,
    #[serde(rename = "@gridFramePenColor")]
    grid_frame_pen_color: String,
    #[serde(rename = "@blendMode")]
    blend_mode: u32,
    #[serde(rename = "@uuid")]
    uuid: QgisUuid,
    #[serde(rename = "@leftAnnotationDirection")]
    left_annotation_direction: u32,
    #[serde(rename = "@position")]
    position: u32,
    #[serde(rename = "@topAnnotationDirection")]
    top_annotation_direction: u32,
    #[serde(rename = "@gridStyle")]
    grid_style: u32,
    #[serde(rename = "@bottomFrameDivisions")]
    bottom_frame_divisions: u32,
    #[serde(rename = "@topAnnotationDisplay")]
    top_annotation_display: u32,
    #[serde(rename = "@rightAnnotationPosition")]
    right_annotation_position: u32,
    #[serde(rename = "@topFrameDivisions")]
    top_frame_divisions: u32,
    #[serde(rename = "@bottomAnnotationDisplay")]
    bottom_annotation_display: u32,
    #[serde(rename = "@rightAnnotationDirection")]
    right_annotation_direction: u32,
    #[serde(rename = "@rotatedTicksLengthMode")]
    rotated_ticks_length_mode: u32,
    #[serde(rename = "@frameAnnotationDistance")]
    frame_annotation_distance: f64,
    #[serde(rename = "@leftFrameDivisions")]
    left_frame_divisions: u32,
    #[serde(rename = "@rightFrameDivisions")]
    right_frame_divisions: u32,
    #[serde(rename = "@rotatedTicksEnabled")]
    rotated_ticks_enabled: u32,
    #[serde(rename = "@rotatedTicksMinimumAngle")]
    rotated_ticks_minimum_angle: f64,
    #[serde(rename = "@bottomAnnotationPosition")]
    bottom_annotation_position: u32,
    #[serde(rename = "@topAnnotationPosition")]
    top_annotation_position: u32,
    #[serde(rename = "@gridFrameSideFlags")]
    grid_frame_side_flags: u32,
    #[serde(rename = "@crossLength")]
    cross_length: f64,
    #[serde(rename = "@show")]
    show: u32,
    #[serde(rename = "lineStyle")]
    line_style: MapGridLineStyle,
    #[serde(rename = "markerStyle")]
    marker_style: MarkerStyle,
    #[serde(rename = "text-style")]
    text_style: TextStyle,
    #[serde(rename = "LayoutObject")]
    layout_object: LayoutObject,
}

#[derive(Serialize)]
struct MapGridLineStyle {
    symbol: Symbol,
}

#[derive(Serialize)]
struct MarkerStyle {
    symbol: Symbol,
}

impl Default for ComposerMapGrid {
    fn default() -> Self {
        let layout_object = LayoutObject {
            data_defined_properties: DataDefinedProperties::new_with_extra_option(),
            custom_properties: Some(CustomProperties { option: None }),
        };
        let text_style = TextStyle::new(7, None, Color::grey());

        Self {
            grid_frame_margin: 0.0,
            left_annotation_position: 1,
            rotated_ticks_margin_to_corner: 0.0,
            interval_x: 1000.0,
            left_annotation_display: 0,
            offset_x: 0.0,
            show_annotation: 1,
            annotation_precision: 0,
            rotated_annotations_enabled: 0,
            right_annotation_display: 3,
            grid_frame_width: 2.0,
            name: "map_ticks".to_string(),
            interval_y: 1000.0,
            rotated_annotations_minimum_angle: 0.0,
            unit: 0,
            bottom_annotation_direction: 0,
            minimum_interval_width: 50.0,
            annotation_format: 0,
            rotated_annotations_margin_to_corner: 10.0,
            offset_y: 0.0,
            grid_frame_pen_thickness: 0.3,
            annotation_expression: String::new(),
            frame_fill_color1: "255,255,255,255".to_string(),
            maximum_interval_width: 100.0,
            rotated_annotations_length_mode: 0,
            frame_fill_color2: "0,0,0,255".to_string(),
            grid_frame_style: 2,
            grid_frame_pen_color: "0,0,0,255".to_string(),
            blend_mode: 0,
            uuid: QgisUuid(uuid::Uuid::new_v4()),
            left_annotation_direction: 3,
            position: 3,
            top_annotation_direction: 0,
            grid_style: 3,
            bottom_frame_divisions: 0,
            top_annotation_display: 3,
            right_annotation_position: 1,
            top_frame_divisions: 0,
            bottom_annotation_display: 0,
            right_annotation_direction: 0,
            rotated_ticks_length_mode: 0,
            frame_annotation_distance: 1.0,
            left_frame_divisions: 0,
            right_frame_divisions: 0,
            rotated_ticks_enabled: 0,
            rotated_ticks_minimum_angle: 0.0,
            bottom_annotation_position: 1,
            top_annotation_position: 1,
            grid_frame_side_flags: 15,
            cross_length: 3.0,
            show: 1,
            layout_object,
            line_style: MapGridLineStyle {
                symbol: Symbol::line_symbol(LineJoinStyle::Bevel, LineCapStyle::Flat),
            },
            marker_style: MarkerStyle {
                symbol: Symbol::marker_symbol(),
            },
            text_style,
        }
    }
}

impl ComposerMapGrid {
    pub fn new(grid_spacing: u32) -> Self {
        ComposerMapGrid {
            interval_x: grid_spacing as f64,
            interval_y: grid_spacing as f64,
            ..Default::default()
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::qgis::{
        layout::components::layout_item::components::ComposerMapGrid,
        tests::test_helpers::{SkipNode, xml_comparison},
    };

    #[test]
    fn map_grid_works() {
        let map_grid = ComposerMapGrid::default();
        let xml = quick_xml::se::to_string(&map_grid).expect("failed to convert map grid to xml");
        let expected = include_str!("../../examples/map_grid.xml");
        let skip_config = vec![SkipNode {
            node_name: "text-style".into(),
            attributes_to_skip: vec![],
            children_to_skip: vec!["background".into()],
            skip_text: false,
        }];
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
