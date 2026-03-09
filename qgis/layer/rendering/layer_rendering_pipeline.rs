use serde::Serialize;

#[derive(Serialize, Clone, Default)]
#[allow(non_snake_case)]
pub struct LayerRenderingPipeline {
    provider: RenderingProvider,
    rasterrenderer: RasterRenderer,
    pub brightnesscontrast: BrightnessContrast,
    pub huesaturation: HueSaturation,
    pub rasterresampler: RasterResampler,
    resamplingStage: ResamplingStage,
}

#[derive(Serialize, Clone, Default)]
struct RenderingProvider {
    resampling: Resampling,
}

#[derive(Serialize, Clone)]
struct Resampling {
    #[serde(rename = "@enabled")]
    enabled: bool,
    #[serde(rename = "@zoomedOutResamplingMethod")]
    zoomed_out_resampling_method: String,
    #[serde(rename = "@maxOversampling")]
    max_oversampling: String,
    #[serde(rename = "@zoomedInResamplingMethod")]
    zoomed_in_resampling_method: String,
}

impl Default for Resampling {
    fn default() -> Self {
        Self {
            enabled: false,
            zoomed_out_resampling_method: "nearestNeighbour".to_string(),
            max_oversampling: "2".to_string(),
            zoomed_in_resampling_method: "nearestNeighbour".to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
struct RasterRenderer {
    #[serde(rename = "@opacity")]
    opacity: String,
    #[serde(rename = "@nodataColor")]
    nodata_color: String,
    #[serde(rename = "@band")]
    band: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@alphaBand")]
    alpha_band: String,
    #[serde(rename = "rasterTransparency")]
    raster_transparency: RasterTransparency,
    #[serde(rename = "minMaxOrigin")]
    min_max_origin: MinMaxOrigin,
}

#[derive(Serialize, Clone)]
struct RasterTransparency {}

impl Default for RasterRenderer {
    fn default() -> Self {
        Self {
            opacity: "1".to_string(),
            nodata_color: "".to_string(),
            band: "1".to_string(),
            r#type: "singlebandcolordata".to_string(),
            alpha_band: "-1".to_string(),
            raster_transparency: RasterTransparency {},
            min_max_origin: MinMaxOrigin::default(),
        }
    }
}

#[derive(Serialize, Clone)]
struct MinMaxOrigin {
    limits: String,
    extent: String,
    #[serde(rename = "statAccuracy")]
    stat_accuracy: String,
    #[serde(rename = "cumulativeCutLower")]
    cumulative_cut_lower: String,
    #[serde(rename = "cumulativeCutUpper")]
    cumulative_cut_upper: String,
    #[serde(rename = "stdDevFactor")]
    std_dev_factor: String,
}

impl Default for MinMaxOrigin {
    fn default() -> Self {
        Self {
            limits: "None".to_string(),
            extent: "WholeRaster".to_string(),
            stat_accuracy: "Estimated".to_string(),
            cumulative_cut_lower: "0.02".to_string(),
            cumulative_cut_upper: "0.98".to_string(),
            std_dev_factor: "2".to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct BrightnessContrast {
    #[serde(rename = "@gamma")]
    gamma: f32,
    #[serde(rename = "@brightness")]
    brightness: f32,
    #[serde(rename = "@contrast")]
    constrast: f32,
}

#[derive(Serialize, Clone)]
#[allow(non_snake_case)]
pub struct HueSaturation {
    #[serde(rename = "@invertColors")]
    pub invertColors: u16,
    #[serde(rename = "@saturation")]
    pub saturation: f32,
    #[serde(rename = "@colorizeStrength")]
    pub colorizeStrength: f32,
    #[serde(rename = "@colorizeOn")]
    pub colorizeOn: u8,
    #[serde(rename = "@colorizeBlue")]
    pub colorizeBlue: u16,
    #[serde(rename = "@colorizeGreen")]
    pub colorizeGreen: u16,
    #[serde(rename = "@grayscaleMode")]
    pub greyscaleMode: u16,
    #[serde(rename = "@colorizeRed")]
    pub colorizeRed: u16,
}

impl Default for HueSaturation {
    fn default() -> Self {
        Self {
            invertColors: Default::default(),
            saturation: Default::default(),
            colorizeStrength: 100.,
            colorizeOn: Default::default(),
            colorizeBlue: 128,
            colorizeGreen: 128,
            greyscaleMode: Default::default(),
            colorizeRed: 255,
        }
    }
}

impl Default for BrightnessContrast {
    fn default() -> Self {
        Self {
            gamma: 1.,
            brightness: 0.,
            constrast: 0.,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct RasterResampler {
    #[serde(rename = "@maxOversampling")]
    max_oversampling: String,
}

impl Default for RasterResampler {
    fn default() -> Self {
        Self {
            max_oversampling: "2".to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
struct ResamplingStage {
    #[serde(rename = "$text")]
    content: String,
}

impl Default for ResamplingStage {
    fn default() -> Self {
        Self {
            content: "resamplingFilter".to_string(),
        }
    }
}
