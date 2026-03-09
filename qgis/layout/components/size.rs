use serde::Serialize;

use crate::layout::PageSize;

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub width_mm: f64,
    pub height_mm: f64,
}

impl Serialize for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let formatted = format!("{},{},mm", self.width_mm, self.height_mm,);
        serializer.serialize_str(&formatted)
    }
}

pub struct SizeInteger {
    pub width_mm: u32,
    pub height_mm: u32,
}

impl From<PageSize> for Size {
    fn from(page_size: PageSize) -> Self {
        let SizeInteger {
            width_mm,
            height_mm,
        } = page_size.size_mm();
        Size {
            width_mm: width_mm as f64,
            height_mm: height_mm as f64,
        }
    }
}
