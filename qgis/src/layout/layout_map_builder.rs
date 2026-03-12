use crate::qgis::{
    Extent,
    enums::EPSGID,
    layout::{
        Size,
        components::{ComposerMapGrid, LayoutItem, Position},
    },
};

pub struct QgisLayoutMapBuilder {
    pub size: Size,
    pub position: Position,
    pub extent: Extent,
    pub srs: Option<EPSGID>,
    pub uuid: uuid::Uuid,
    pub id: String,
    pub map_grid: Option<ComposerMapGrid>,
    pub keep_layer_set: bool,
    pub map_to_overview: Option<uuid::Uuid>,
    pub z_value: Option<u32>,
}

impl QgisLayoutMapBuilder {
    pub fn new_bng(uuid: uuid::Uuid, id: String) -> QgisLayoutMapBuilder {
        let srs = EPSGID::BNG;
        Self {
            size: Size {
                width_mm: 138.,
                height_mm: 195.,
            },
            position: Position { x: 9.907, y: 8.294 },
            extent: srs.default_extent(),
            srs: Some(srs),
            uuid,
            id,
            map_grid: None,
            keep_layer_set: false,
            map_to_overview: None,
            z_value: None,
        }
    }
    pub fn keep_layer_set(&mut self) {
        self.keep_layer_set = true
    }

    pub fn build(self) -> LayoutItem {
        let mut map = match self.map_to_overview {
            Some(frame_map) => LayoutItem::map_with_overview(
                self.size,
                self.position,
                self.extent.into(),
                self.srs.map(|srs| srs.qgis_srs()),
                self.uuid,
                self.id,
                frame_map,
            ),
            None => LayoutItem::map(
                self.size,
                self.position,
                self.extent.into(),
                self.srs.map(|srs| srs.qgis_srs()),
                self.uuid,
                self.id,
                self.map_grid,
            ),
        };
        map.keep_layer_set = Some(self.keep_layer_set);
        if let Some(z_value) = self.z_value {
            map.set_z_value(z_value);
        }
        map
    }
}
