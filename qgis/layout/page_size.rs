use crate::qgis::layout::components::SizeInteger;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone)]
pub enum PageSize {
    A0(PageOrientation),
    A1(PageOrientation),
    A2(PageOrientation),
    A3(PageOrientation),
    A4(PageOrientation),
}

#[derive(Default, Copy, Clone, EnumIter)]
pub enum PageOrientation {
    Portrait,
    #[default]
    Landscape,
}

impl Default for PageSize {
    fn default() -> Self {
        PageSize::A3(PageOrientation::default())
    }
}

impl PageSize {
    /// Returns the size in millimeters for portrait orientation
    pub fn portrait_size_mm(&self) -> SizeInteger {
        match self {
            PageSize::A0(_) => SizeInteger {
                width_mm: 841,
                height_mm: 1189,
            },
            PageSize::A1(_) => SizeInteger {
                width_mm: 594,
                height_mm: 841,
            },
            PageSize::A2(_) => SizeInteger {
                width_mm: 420,
                height_mm: 594,
            },
            PageSize::A3(_) => SizeInteger {
                width_mm: 297,
                height_mm: 420,
            },
            PageSize::A4(_) => SizeInteger {
                width_mm: 210,
                height_mm: 297,
            },
        }
    }

    pub fn from_dimensions(size: SizeInteger) -> Option<PageSize> {
        for page_variant in PageSize::iter() {
            for orientation in PageOrientation::iter() {
                let page_size = match page_variant {
                    PageSize::A0(_) => PageSize::A0(orientation),
                    PageSize::A1(_) => PageSize::A1(orientation),
                    PageSize::A2(_) => PageSize::A2(orientation),
                    PageSize::A3(_) => PageSize::A3(orientation),
                    PageSize::A4(_) => PageSize::A4(orientation),
                };
                let page_size = page_size.size_mm();

                if page_size.width_mm == size.width_mm && page_size.height_mm == size.height_mm {
                    return Some(page_variant);
                }
            }
        }
        None
    }

    /// Returns the actual size considering orientation
    pub fn size_mm(&self) -> SizeInteger {
        let portrait = self.portrait_size_mm();
        match self.orientation() {
            PageOrientation::Portrait => portrait,
            PageOrientation::Landscape => SizeInteger {
                width_mm: portrait.height_mm,
                height_mm: portrait.width_mm,
            },
        }
    }

    pub fn orientation(&self) -> &PageOrientation {
        match self {
            PageSize::A0(orientation) => orientation,
            PageSize::A1(orientation) => orientation,
            PageSize::A2(orientation) => orientation,
            PageSize::A3(orientation) => orientation,
            PageSize::A4(orientation) => orientation,
        }
    }

    pub fn name(&self) -> String {
        match self {
            PageSize::A0(_) => "A0".into(),
            PageSize::A1(_) => "A1".into(),
            PageSize::A2(_) => "A2".into(),
            PageSize::A3(_) => "A3".into(),
            PageSize::A4(_) => "A4".into(),
        }
    }
}
