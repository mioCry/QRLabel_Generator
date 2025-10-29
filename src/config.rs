use serde::{Deserialize, Serialize};

/// Configuration for QR code generation
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Default size in pixels for generated QR codes
    pub default_size: u32,
    /// Error correction level: "L", "M", "Q", or "H"
    pub error_correction_level: String,
    /// Output format (currently only "png" is supported)
    pub output_format: String,
    /// Foreground color as RGB [r, g, b]
    pub foreground_color: [u8; 3],
    /// Background color as RGB [r, g, b]
    pub background_color: [u8; 3],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_size: 200,
            error_correction_level: "M".to_string(),
            output_format: "png".to_string(),
            foreground_color: [0, 0, 0],      // Black
            background_color: [255, 255, 255], // White
        }
    }
}
