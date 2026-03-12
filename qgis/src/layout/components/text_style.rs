use serde::{Deserialize, Serialize};

use crate::{
    data_defined_properties::{DataDefinedProperties, PropertiesOption, PropertyOption},
    enums::NamedTextStyle,
    layout::components::Color,
};

#[derive(Serialize)]
#[serde(rename = "text-style")]
pub struct TextStyle {
    #[serde(rename = "@textColor")]
    pub text_color: String,
    #[serde(rename = "@fontItalic")]
    font_italic: u32,
    #[serde(rename = "@fontWordSpacing")]
    font_word_spacing: u32,
    #[serde(rename = "@textOpacity")]
    text_opacity: u32,
    #[serde(rename = "@blendMode")]
    blend_mode: u32,
    #[serde(rename = "@fontStrikeout")]
    font_strikeout: u32,
    #[serde(rename = "@textOrientation")]
    text_orientation: String,
    #[serde(rename = "@fontSizeUnit")]
    font_size_unit: String,
    #[serde(rename = "@fontSize")]
    pub font_size: u16,
    #[serde(rename = "@allowHtml")]
    allow_html: u32,
    #[serde(rename = "@multilineHeight")]
    pub multiline_height: f32,
    #[serde(rename = "@fontUnderline")]
    font_underline: u32,
    #[serde(rename = "@multilineHeightUnit")]
    multiline_height_unit: String,
    #[serde(rename = "@capitalization")]
    capitalization: u32,
    #[serde(rename = "@forcedItalic")]
    forced_italic: u32,
    #[serde(rename = "@previewBkgrdColor")]
    preview_bkgrd_color: String,
    #[serde(rename = "@fontKerning")]
    font_kerning: u32,
    #[serde(rename = "@namedStyle")]
    pub named_style: String,
    #[serde(rename = "@forcedBold")]
    forced_bold: u32,
    #[serde(rename = "@fontSizeMapUnitScale")]
    font_size_map_unit_scale: String,
    #[serde(rename = "@fontFamily")]
    font_family: String,
    #[serde(rename = "@fontWeight")]
    font_weight: u32,
    #[serde(rename = "@fontLetterSpacing")]
    font_letter_spacing: u32,
    #[serde(rename = "families")]
    families: Families,
    #[serde(rename = "text-buffer")]
    pub text_buffer: TextBuffer,
    #[serde(rename = "text-mask")]
    text_mask: TextMask,
    #[serde(rename = "background")]
    background: TextBackground,
    #[serde(rename = "shadow")]
    shadow: TextShadow,
    #[serde(rename = "dd_properties")]
    dd_properties: DataDefinedProperties,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Families {}

#[derive(Serialize, Default)]
#[serde(rename = "text-buffer")]
pub struct TextBuffer {
    #[serde(rename = "@bufferNoFill")]
    pub buffer_no_fill: u32,
    #[serde(rename = "@bufferJoinStyle")]
    pub buffer_join_style: u32,
    #[serde(rename = "@bufferSizeMapUnitScale")]
    pub buffer_size_map_unit_scale: String,
    #[serde(rename = "@bufferColor")]
    pub buffer_color: String,
    #[serde(rename = "@bufferDraw")]
    pub buffer_draw: u32,
    #[serde(rename = "@bufferSize")]
    pub buffer_size: f32,
    #[serde(rename = "@bufferBlendMode")]
    pub buffer_blend_mode: u32,
    #[serde(rename = "@bufferSizeUnits")]
    pub buffer_size_units: String,
    #[serde(rename = "@bufferOpacity")]
    pub buffer_opacity: f32,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "text-mask")]
pub struct TextMask {
    #[serde(rename = "@maskSizeUnits")]
    mask_size_units: String,
    #[serde(rename = "@maskSizeMapUnitScale")]
    mask_size_map_unit_scale: String,
    #[serde(rename = "@maskEnabled")]
    mask_enabled: u32,
    #[serde(rename = "@maskedSymbolLayers")]
    masked_symbol_layers: String,
    #[serde(rename = "@maskType")]
    mask_type: u32,
    #[serde(rename = "@maskJoinStyle")]
    mask_join_style: u32,
    #[serde(rename = "@maskSize")]
    mask_size: String,
    #[serde(rename = "@maskOpacity")]
    mask_opacity: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TextBackground {
    #[serde(rename = "@shapeSizeUnit")]
    shape_size_unit: String,
    #[serde(rename = "@shapeSVGFile")]
    shape_svg_file: String,
    #[serde(rename = "@shapeBorderWidth")]
    shape_border_width: u32,
    #[serde(rename = "@shapeOffsetUnit")]
    shape_offset_unit: String,
    #[serde(rename = "@shapeSizeMapUnitScale")]
    shape_size_map_unit_scale: String,
    #[serde(rename = "@shapeSizeX")]
    shape_size_x: u32,
    #[serde(rename = "@shapeOffsetY")]
    shape_offset_y: u32,
    #[serde(rename = "@shapeOpacity")]
    shape_opacity: u32,
    #[serde(rename = "@shapeOffsetX")]
    shape_offset_x: u32,
    #[serde(rename = "@shapeSizeY")]
    shape_size_y: u32,
    #[serde(rename = "@shapeRotation")]
    shape_rotation: u32,
    #[serde(rename = "@shapeRotationType")]
    shape_rotation_type: u32,
    #[serde(rename = "@shapeRadiiMapUnitScale")]
    shape_radii_map_unit_scale: String,
    #[serde(rename = "@shapeRadiiY")]
    shape_radii_y: u32,
    #[serde(rename = "@shapeRadiiUnit")]
    shape_radii_unit: String,
    #[serde(rename = "@shapeType")]
    shape_type: u32,
    #[serde(rename = "@shapeOffsetMapUnitScale")]
    shape_offset_map_unit_scale: String,
    #[serde(rename = "@shapeSizeType")]
    shape_size_type: u32,
    #[serde(rename = "@shapeFillColor")]
    shape_fill_color: String,
    #[serde(rename = "@shapeRadiiX")]
    shape_radii_x: u32,
    #[serde(rename = "@shapeJoinStyle")]
    shape_join_style: u32,
    #[serde(rename = "@shapeDraw")]
    shape_draw: u32,
    #[serde(rename = "@shapeBorderColor")]
    shape_border_color: String,
    #[serde(rename = "@shapeBorderWidthMapUnitScale")]
    shape_border_width_map_unit_scale: String,
    #[serde(rename = "@shapeBorderWidthUnit")]
    shape_border_width_unit: String,
    #[serde(rename = "@shapeBlendMode")]
    shape_blend_mode: u32,
    #[serde(rename = "symbol")]
    symbol: BackgroundSymbol,
}

#[derive(Serialize, Deserialize)]
pub struct BackgroundSymbol {
    #[serde(rename = "@force_rhr")]
    force_rhr: u32,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@alpha")]
    alpha: u32,
    #[serde(rename = "@type")]
    symbol_type: String,
    #[serde(rename = "@is_animated")]
    is_animated: u32,
    #[serde(rename = "@clip_to_extent")]
    clip_to_extent: u32,
    #[serde(rename = "@frame_rate")]
    frame_rate: u32,
    #[serde(rename = "data_defined_properties")]
    data_defined_properties: DataDefinedProperties,
    #[serde(rename = "layer")]
    layer: SymbolLayer,
}

#[derive(Serialize, Deserialize)]
pub struct SymbolLayer {
    #[serde(rename = "@class")]
    class: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@locked")]
    locked: u32,
    #[serde(rename = "@enabled")]
    enabled: u32,
    #[serde(rename = "@pass")]
    pass: u32,
    #[serde(rename = "Option")]
    option: LayerOption,
    #[serde(rename = "data_defined_properties")]
    data_defined_properties: DataDefinedProperties,
}

impl Default for SymbolLayer {
    fn default() -> Self {
        Self {
            class: "SimpleFill".into(),
            id: Default::default(),
            locked: Default::default(),
            enabled: 1,
            pass: Default::default(),
            option: Default::default(),
            data_defined_properties: DataDefinedProperties {
                option: PropertiesOption {
                    option_type: "Map".to_string(),
                    options: vec![],
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LayerOption {
    #[serde(rename = "@type")]
    option_type: String,
    // Contains multiple Option elements with different names and values
}

impl Default for LayerOption {
    fn default() -> Self {
        Self {
            option_type: "Map".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TextShadow {
    #[serde(rename = "@shadowUnder")]
    shadow_under: String,
    #[serde(rename = "@shadowOffsetMapUnitScale")]
    shadow_offset_map_unit_scale: String,
    #[serde(rename = "@shadowRadiusMapUnitScale")]
    shadow_radius_map_unit_scale: String,
    #[serde(rename = "@shadowColor")]
    shadow_color: String,
    #[serde(rename = "@shadowRadiusUnit")]
    shadow_radius_unit: String,
    #[serde(rename = "@shadowScale")]
    shadow_scale: String,
    #[serde(rename = "@shadowRadius")]
    shadow_radius: String,
    #[serde(rename = "@shadowOffsetUnit")]
    shadow_offset_unit: String,
    #[serde(rename = "@shadowBlendMode")]
    shadow_blend_mode: String,
    #[serde(rename = "@shadowOpacity")]
    shadow_opacity: String,
    #[serde(rename = "@shadowOffsetAngle")]
    shadow_offset_angle: String,
    #[serde(rename = "@shadowRadiusAlphaOnly")]
    shadow_radius_alpha_only: String,
    #[serde(rename = "@shadowOffsetGlobal")]
    shadow_offset_global: String,
    #[serde(rename = "@shadowDraw")]
    shadow_draw: String,
    #[serde(rename = "@shadowOffsetDist")]
    shadow_offset_dist: String,
}

impl Default for TextStyle {
    fn default() -> Self {
        let extra_option = PropertyOption {
            value: None,
            prop_type: None,
            name: "properties".into(),
        };
        let mut text_style = Self {
            text_color: "0,0,0,255".to_string(),
            font_italic: 0,
            font_word_spacing: 0,
            text_opacity: 1,
            blend_mode: 0,
            font_strikeout: 0,
            text_orientation: "horizontal".to_string(),
            font_size_unit: "Point".to_string(),
            font_size: 10,
            allow_html: 0,
            multiline_height: 1.,
            font_underline: 0,
            multiline_height_unit: "Percentage".to_string(),
            capitalization: 0,
            forced_italic: 0,
            preview_bkgrd_color: "255,255,255,255".to_string(),
            font_kerning: 1,
            named_style: "".to_string(),
            forced_bold: 0,
            font_size_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            font_family: "Ubuntu Sans".to_string(),
            font_weight: 50,
            font_letter_spacing: 0,
            families: Families::default(),
            text_buffer: TextBuffer::default_disabled(),
            text_mask: TextMask::default_disabled(),
            background: TextBackground::default_disabled(),
            shadow: TextShadow::default_disabled(),
            dd_properties: DataDefinedProperties::default(),
        };
        text_style
            .dd_properties
            .option
            .options
            .push(extra_option.clone());
        text_style
            .background
            .symbol
            .data_defined_properties
            .option
            .options
            .push(extra_option);
        text_style
    }
}

impl TextStyle {
    pub fn new(font_size: u16, named_text_style: Option<NamedTextStyle>, colour: Color) -> Self {
        TextStyle {
            font_size,
            named_style: named_text_style.map(|s| s.to_string()).unwrap_or("".into()),
            text_color: colour.to_string(),
            ..Default::default()
        }
    }
}

impl TextBuffer {
    pub fn default_disabled() -> Self {
        Self {
            buffer_no_fill: 1,
            buffer_join_style: 128,
            buffer_size_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            buffer_color: Color::white().to_string(),
            buffer_draw: 0,
            buffer_size: 1.,
            buffer_blend_mode: 0,
            buffer_size_units: "MM".to_string(),
            buffer_opacity: 1.,
        }
    }
    pub fn default_enabled() -> Self {
        let mut buffer = Self::default_disabled();
        buffer.buffer_draw = 1;
        buffer
    }
}

impl TextMask {
    pub fn default_disabled() -> Self {
        Self {
            mask_size_units: "MM".to_string(),
            mask_size_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            mask_enabled: 0,
            masked_symbol_layers: "".to_string(),
            mask_type: 0,
            mask_join_style: 128,
            mask_size: "1.5".to_string(),
            mask_opacity: 1,
        }
    }
}

impl TextBackground {
    pub fn default_disabled() -> Self {
        Self {
            shape_size_unit: "MM".to_string(),
            shape_svg_file: "".to_string(),
            shape_border_width: 0,
            shape_offset_unit: "MM".to_string(),
            shape_size_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            shape_size_x: 0,
            shape_offset_y: 0,
            shape_opacity: 1,
            shape_offset_x: 0,
            shape_size_y: 0,
            shape_rotation: 0,
            shape_rotation_type: 0,
            shape_radii_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            shape_radii_y: 0,
            shape_radii_unit: "MM".to_string(),
            shape_type: 0,
            shape_offset_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            shape_size_type: 0,
            shape_fill_color: "255,255,255,255".to_string(),
            shape_radii_x: 0,
            shape_join_style: 64,
            shape_draw: 0,
            shape_border_color: "128,128,128,255".to_string(),
            shape_border_width_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            shape_border_width_unit: "MM".to_string(),
            shape_blend_mode: 0,
            symbol: BackgroundSymbol::default(),
        }
    }
}

impl TextShadow {
    pub fn default_disabled() -> Self {
        Self {
            shadow_under: "0".to_string(),
            shadow_offset_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            shadow_radius_map_unit_scale: "3x:0,0,0,0,0,0".to_string(),
            shadow_color: "0,0,0,255".to_string(),
            shadow_radius_unit: "MM".to_string(),
            shadow_scale: "100".to_string(),
            shadow_radius: "1.5".to_string(),
            shadow_offset_unit: "MM".to_string(),
            shadow_blend_mode: "6".to_string(),
            shadow_opacity: "0.69999999999999996".to_string(),
            shadow_offset_angle: "135".to_string(),
            shadow_radius_alpha_only: "0".to_string(),
            shadow_offset_global: "1".to_string(),
            shadow_draw: "0".to_string(),
            shadow_offset_dist: "1".to_string(),
        }
    }
}

impl Default for BackgroundSymbol {
    fn default() -> Self {
        Self {
            force_rhr: 0,
            name: "fillSymbol".to_string(),
            alpha: 1,
            symbol_type: "fill".to_string(),
            is_animated: 0,
            clip_to_extent: 1,
            frame_rate: 10,
            data_defined_properties: DataDefinedProperties::default(),
            layer: SymbolLayer::default(),
        }
    }
}
