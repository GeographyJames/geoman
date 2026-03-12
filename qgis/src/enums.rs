use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Extent, srs::SpatialRefSys};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ReferencePoint {
    #[default]
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    MiddleLeft = 3,
    MiddleCenter = 4,
    MiddleRight = 5,
    BottomLeft = 6,
    BottomCenter = 7,
    BottomRight = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum HorizontalAlignment {
    #[default]
    Left = 1,
    Center = 4,
    Right = 2,
    Justify = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum VerticalAlignment {
    #[default]
    Top = 32,
    Middle = 128,
    Bottom = 64,
}

impl Serialize for ReferencePoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl Serialize for VerticalAlignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl Serialize for HorizontalAlignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}
#[derive(Default)]
pub enum LineJoinStyle {
    Bevel,
    #[default]
    Miter,
}

impl Display for LineJoinStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bevel => write!(f, "bevel"),
            Self::Miter => write!(f, "miter"),
        }
    }
}

impl Serialize for LineJoinStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[derive(Default, Clone)]
pub enum Units {
    #[default]
    Millimeter,
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Millimeter => write!(f, "mm"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum ScalebarUnits {
    #[default]
    Kilometers,
    Meters,
}

impl Display for ScalebarUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kilometers => write!(f, "km"),
            Self::Meters => write!(f, "m"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum NamedTextStyle {
    #[default]
    Regular,
    Bold,
    MediumItalic,
}

impl Display for NamedTextStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bold => write!(f, "Bold"),
            Self::MediumItalic => write!(f, "Medium Italic"),
            Self::Regular => write!(f, "Regular"),
        }
    }
}

#[derive(Serialize, Default)]

pub enum LineCapStyle {
    #[default]
    Square,
    Flat,
}

impl Display for LineCapStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Square => write!(f, "square"),
            Self::Flat => write!(f, "flat"),
        }
    }
}

#[derive(Serialize, Clone, Default)]
#[allow(non_camel_case_types)]
pub enum MapUnits {
    meters,
    #[default]
    degrees,
}

#[derive(Clone, Copy)]
pub enum EPSGID {
    BNG = 27700,
    WGS84 = 4326,
}

impl EPSGID {
    pub fn map_units(&self) -> MapUnits {
        match self {
            Self::BNG => MapUnits::meters,
            Self::WGS84 => MapUnits::degrees,
        }
    }
    pub fn qgis_srs(&self) -> SpatialRefSys {
        match self {
            Self::BNG => SpatialRefSys::bng(),
            Self::WGS84 => SpatialRefSys::wgs84(),
        }
    }
    pub fn default_extent(&self) -> Extent {
        match self {
            Self::BNG => Extent::uk(),
            Self::WGS84 => Extent::wgs84(),
        }
    }
}
