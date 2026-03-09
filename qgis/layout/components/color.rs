use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct Color {
    #[serde(rename = "@red")]
    pub red: u32,
    #[serde(rename = "@blue")]
    pub blue: u32,
    #[serde(rename = "@alpha")]
    pub alpha: u32,
    #[serde(rename = "@green")]
    pub green: u32,
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.red, self.blue, self.green, self.alpha
        )
    }
}

impl Color {
    pub fn white() -> Self {
        Self {
            red: 255,
            blue: 255,
            alpha: 255,
            green: 255,
        }
    }
    pub fn black() -> Self {
        Self {
            red: 0,
            blue: 0,
            alpha: 255,
            green: 0,
        }
    }
    pub fn grey() -> Self {
        Self {
            red: 128,
            blue: 128,
            green: 128,
            alpha: 255,
        }
    }
}
