use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_size: u32,
    pub error_correction_level: String,
    pub output_format: String,
    pub foreground_color: [u8; 3],
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
