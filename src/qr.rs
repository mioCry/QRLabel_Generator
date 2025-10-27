use anyhow::Result;
use qrcode::QrCode;
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::warehouse::WarehouseItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct QrData {
    pub text: String,
    pub filename: String,
    pub size: Option<u32>,
}

pub struct QrGenerator {
    error_correction_level: qrcode::EcLevel,
    foreground_color: Rgb<u8>,
    background_color: Rgb<u8>,
}

impl Default for QrGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl QrGenerator {
    /// Creates a generator with default settings (EC=M, black/white colors)
    pub fn new() -> Self {
        Self {
            error_correction_level: qrcode::EcLevel::M,
            foreground_color: Rgb([0, 0, 0]),
            background_color: Rgb([255, 255, 255]),
        }
    }

    /// Creates a generator from a `Config`
    pub fn from_config(config: &Config) -> Self {
        let level = match config.error_correction_level.as_str() {
            "L" | "l" => qrcode::EcLevel::L,
            "M" | "m" => qrcode::EcLevel::M,
            "Q" | "q" => qrcode::EcLevel::Q,
            "H" | "h" => qrcode::EcLevel::H,
            _ => qrcode::EcLevel::M,
        };
        Self {
            error_correction_level: level,
            foreground_color: Rgb(config.foreground_color),
            background_color: Rgb(config.background_color),
        }
    }

    pub fn generate_single(&self, text: &str, output_path: &str, size: u32) -> Result<()> {
        let code = QrCode::with_error_correction_level(text, self.error_correction_level)?;
        let image = self.qr_to_image(&code, size);
        image.save(output_path)?;
        Ok(())
    }

    pub fn generate_batch(&self, input_file: &str, output_dir: &str) -> Result<()> {
        // Create output folder if it doesn't exist
        fs::create_dir_all(output_dir)?;
        
        let content = fs::read_to_string(input_file)?;
        let qr_data_list: Vec<QrData> = serde_json::from_str(&content)?;
        
        for qr_data in qr_data_list {
            let size = qr_data.size.unwrap_or(200);
            let output_path = format!("{}/{}", output_dir, qr_data.filename);
            self.generate_single(&qr_data.text, &output_path, size)?;
        }
        
        Ok(())
    }

    /// Generates QR codes from generic CSV with headers: text,filename,size
    pub fn generate_batch_from_csv(&self, csv_file: &str, output_dir: &str) -> Result<()> {
        use std::fs;
        fs::create_dir_all(output_dir)?;
        let mut rdr = csv::Reader::from_path(csv_file)?;
        for rec in rdr.deserialize() {
            let row: QrData = rec?; // reuse QrData: { text, filename, size }
            let size = row.size.unwrap_or(200);
            let output_path = format!("{}/{}", output_dir, row.filename);
            self.generate_single(&row.text, &output_path, size)?;
        }
        Ok(())
    }

    /// Generates QR codes from warehouse CSV with headers:
    /// code,description,warehouse_location,shelf,shelf_relative_position,size(optional)
    pub fn generate_warehouse_batch_from_csv(&self, csv_file: &str, output_dir: &str, default_size: Option<u32>) -> Result<()> {
        use std::fs;
        fs::create_dir_all(output_dir)?;
        let mut rdr = csv::Reader::from_path(csv_file)?;
        for rec in rdr.deserialize() {
            #[derive(Deserialize)]
            struct Row {
                code: String,
                description: String,
                warehouse_location: String,
                shelf: u32,
                shelf_relative_position: u32,
                size: Option<u32>,
            }
            let row: Row = rec?;
            let item = WarehouseItem {
                code: row.code,
                description: row.description,
                warehouse_location: row.warehouse_location,
                shelf: row.shelf,
                shelf_relative_position: row.shelf_relative_position,
            };
            let size = row.size.or(default_size).unwrap_or(200);
            let filename = format!("{}_{}.png", item.code, item.formatted_shelf_position());
            let output_path = format!("{}/{}", output_dir, filename);
            self.generate_from_warehouse_item(&item, &output_path, size)?;
        }
        Ok(())
    }

    fn qr_to_image(&self, code: &QrCode, size: u32) -> RgbImage {
        let width = code.width();
        let mut image = ImageBuffer::new(width as u32, width as u32);
        
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let module = code[(x as usize, y as usize)];
            *pixel = if module == qrcode::Color::Dark {
                self.foreground_color
            } else {
                self.background_color
            };
        }
        
        // Resize image to desired size
        image::imageops::resize(&image, size, size, image::imageops::FilterType::Nearest)
    }

    /// Generates a QR code for a `WarehouseItem` using the shelf format `Sxxx-Pxxx`
    pub fn generate_from_warehouse_item(&self, item: &WarehouseItem, output_path: &str, size: u32) -> Result<()> {
        let text = item.to_qr_text();
        self.generate_single(&text, output_path, size)
    }

    /// Generates QR codes from a JSON file containing a list of `WarehouseItem`.
    /// The JSON file must be an array of `WarehouseItem` objects.
    pub fn generate_warehouse_batch(&self, input_file: &str, output_dir: &str, default_size: Option<u32>) -> Result<()> {
        use std::fs;
        fs::create_dir_all(output_dir)?;

        let content = fs::read_to_string(input_file)?;
        let items: Vec<WarehouseItem> = serde_json::from_str(&content)?;

        for item in items {
            let size = default_size.unwrap_or(200);
            // build a friendly filename e.g: ART-0001_S001-P001.png
            let filename = format!("{}_{}.png", item.code, item.formatted_shelf_position());
            let output_path = format!("{}/{}", output_dir, filename);
            self.generate_from_warehouse_item(&item, &output_path, size)?;
        }

        Ok(())
    }
}
