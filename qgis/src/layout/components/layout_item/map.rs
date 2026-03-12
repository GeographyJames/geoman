use crate::extent::LayoutMapExtent;
use crate::layout::components::layout_item::Length;
use crate::layout::components::layout_item::components::ComposerMapOverview;

use crate::QgisUuid;
use crate::layout::components::ComposerMapGrid;
use crate::{
    clipping_settings::ClippingSettings,
    enums::Units,
    layout::{
        Size,
        components::{Color, LayoutItem, LayoutObject, Position},
    },
    srs::{SpatialRefSys, Srs},
};

impl LayoutItem {
    pub fn map(
        size: Size,
        position: Position,
        extent: LayoutMapExtent,
        srs: Option<SpatialRefSys>,
        uuid: uuid::Uuid,
        id: String,
        map_grid: Option<ComposerMapGrid>,
    ) -> Self {
        let srs = srs.map(|s| Srs { spatialrefsys: s });
        Self {
            draw_canvas_items: Some(true),
            is_temporal: Some(0),
            item_type: 65639,
            position_on_page: position,
            id,
            size,
            keep_layer_set: Some(false),
            follow_preset: Some(false),
            map_flags: Some(0),
            map_rotation: Some(0),
            follow_preset_name: Some(String::new()),
            position,
            z_value: 1,
            frame: true,
            outline_width_m: Length {
                distance: 0.3,
                units: Units::Millimeter,
            },
            uuid: QgisUuid(uuid),
            opacity: 1.,
            visibility: 1,
            background: true,
            label_margin: Some(Default::default()),
            atlas_map: Some(Default::default()),
            extent: Some(extent),
            layer_set: Some(Default::default()),
            atlas_clipping_settings: Some(ClippingSettings::atlas()),
            item_clipping_settings: Some(ClippingSettings::item()),
            label_blocking_items: Some(Default::default()),
            frame_color: Color::black(),
            layout_object: LayoutObject::default_with_custom_properties_and_extra_option(),
            crs: srs,
            composer_map_grid: map_grid,
            ..Default::default()
        }
    }
    pub fn map_with_overview(
        size: Size,
        position: Position,
        extent: LayoutMapExtent,
        srs: Option<SpatialRefSys>,
        uuid: uuid::Uuid,
        id: String,
        frame_map: uuid::Uuid,
    ) -> Self {
        let mut map = Self::map(size, position, extent, srs, uuid, id, None);
        map.ComposerMapOverview = Some(ComposerMapOverview::new(frame_map));

        map
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        extent::LayoutMapExtent,
        layout::{
            Size,
            components::{LayoutItem, Position},
        },
        srs::SpatialRefSys,
        tests::test_helpers::{SkipNode, xml_comparison},
    };

    #[test]
    fn map_layout_item_works() {
        let xml = quick_xml::se::to_string(&LayoutItem::map(
            Size {
                width_mm: 237.,
                height_mm: 192.,
            },
            Position {
                x: 9.90698,
                y: 8.29422,
            },
            LayoutMapExtent::default(),
            None,
            uuid::Uuid::new_v4(),
            "Map 1".into(),
            None,
        ))
        .expect("failed to serialize xml");

        let expected = r#"<LayoutItem
      opacity="1"
      templateUuid="{4da48fe6-c986-4915-85dd-151215341eee}"
      drawCanvasItems="true"
      frameJoinStyle="miter"
      groupUuid=""
      isTemporal="0"
      visibility="1"
      positionOnPage="9.90698,8.29422,mm"
      size="237,192,mm"
      excludeFromExports="0"
      type="65639"
      zValue="1"
      labelMargin="0,mm"
      referencePoint="0"
      keepLayerSet="false"
      followPreset="false"
      mapFlags="0"
      positionLock="false"
      mapRotation="0"
      followPresetName=""
      id="Map 1"
      background="true"
      outlineWidthM="0.3,mm"
      itemRotation="0"
      position="9.90698,8.29422,mm"
      frame="true"
      blendMode="0"
      uuid="{4da48fe6-c986-4915-85dd-151215341eee}">
        <FrameColor red="0" blue="0" alpha="255" green="0"/>
        <BackgroundColor red="255" blue="255" alpha="255" green="255"/>
        <LayoutObject>
          <dataDefinedProperties>
            <Option type="Map">
              <Option value="" type="QString" name="name"/>
              <Option name="properties"/>
              <Option value="collection" type="QString" name="type"/>
            </Option>
          </dataDefinedProperties>
          <customproperties>
            <Option/>
          </customproperties>
        </LayoutObject>
        <Extent xmax="180" xmin="-180" ymin="-90" ymax="90"/>
        <LayerSet/>
        <AtlasMap margin="0.1" atlasDriven="0" scalingMode="2"/>
        <labelBlockingItems/>
        <atlasClippingSettings enabled="0" restrictLayers="0" clippingType="1" forceLabelsInside="0">
          <layersToClip/>
        </atlasClippingSettings>
        <itemClippingSettings clipSource="" enabled="0" clippingType="1" forceLabelsInside="0"/>
      </LayoutItem>"#;
        xml_comparison(&xml, expected, None);
    }
    #[test]
    fn map_with_overview() {
        let xml = quick_xml::se::to_string(&LayoutItem::map_with_overview(
            Size {
                width_mm: 70.,
                height_mm: 70.,
            },
            Position { x: 345., y: 177.15 },
            LayoutMapExtent {
                xmax: 359636.,
                xmin: 289636.,
                ymax: 708221.,
                ymin: 638221.,
            },
            Some(SpatialRefSys::bng()),
            uuid::Uuid::new_v4(),
            "Overview Map".into(),
            uuid::Uuid::new_v4(),
        ))
        .expect("failed to serialize xml");
        let skip_config = vec![
            SkipNode {
                node_name: "LayoutItem".into(),
                attributes_to_skip: vec![],
                children_to_skip: vec![
                    "atlasClippingSettings".into(),
                    "itemClippingSettings".into(),
                ],
                skip_text: false,
            },
            SkipNode {
                node_name: "spatialrefsys".into(),
                attributes_to_skip: vec!["nativeFormat".into()],
                children_to_skip: vec!["wkt".into()],
                skip_text: false,
            },
        ];

        let expected = include_str!("../examples/map_with_overview.xml");
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
