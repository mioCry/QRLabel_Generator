use anyhow::{Result, Context};
use qrcode::QrCode;
use crate::warehouse::WarehouseItem;
use std::io::Write;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

/// QR code encoding mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QrContentMode {
    /// Article code only (minimum density)
    CodeOnly,
    /// Code + location (medium density)
    CodeAndLocation,
    /// All details in JSON (maximum density)
    FullJson,
}

impl Default for QrContentMode {
    fn default() -> Self {
        QrContentMode::CodeOnly
    }
}

/// Label layout configuration for A4 page
#[derive(Debug, Clone)]
pub struct LabelLayout {
    /// Number of labels per row
    pub labels_per_row: usize,
    /// Number of rows per page
    pub rows_per_page: usize,
    /// QR code size in mm
    pub qr_size_mm: f32,
    /// QR code content mode
    pub qr_content_mode: QrContentMode,
    /// QR code error correction level
    pub qr_error_correction: qrcode::EcLevel,
}

impl Default for LabelLayout {
    fn default() -> Self {
        // Default layout: 3x4 = 12 labels per A4 page
        // Reduced to 4 rows to give ample space for text under QR codes
        Self {
            labels_per_row: 3,
            rows_per_page: 4,
            qr_size_mm: 40.0,
            qr_content_mode: QrContentMode::CodeOnly,
            qr_error_correction: qrcode::EcLevel::L, // Low - optimal for simple QR codes
        }
    }
}

/// PDF generator for warehouse labels
pub struct PdfLabelGenerator {
    layout: LabelLayout,
}

impl PdfLabelGenerator {
    /// Creates a new generator with default layout
    pub fn new() -> Self {
        Self {
            layout: LabelLayout::default(),
        }
    }

    /// Creates a new generator with custom layout
    pub fn with_layout(layout: LabelLayout) -> Self {
        Self {
            layout,
        }
    }

    /// Generates a PDF with labels from a CSV file
    ///
    /// # Arguments
    /// * `csv_file` - Path to CSV file with warehouse item data
    /// * `output_pdf` - Path where the PDF file will be saved
    ///
    /// # CSV Format
    /// The CSV file should have headers: `code,description,warehouse_location,shelf,shelf_relative_position,size`
    pub fn generate_pdf_from_csv(&self, csv_file: &str, output_pdf: &str) -> Result<()> {
        use csv::Reader;
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct Row {
            code: String,
            description: String,
            warehouse_location: String,
            shelf: u32,
            shelf_relative_position: u32,
            #[allow(dead_code)]
            size: Option<u32>,
        }

        let mut rdr = Reader::from_path(csv_file)?;
        let mut items = Vec::new();

        for rec in rdr.deserialize() {
            let row: Row = rec?;
            items.push(WarehouseItem {
                code: row.code,
                description: row.description,
                warehouse_location: row.warehouse_location,
                shelf: row.shelf,
                shelf_relative_position: row.shelf_relative_position,
            });
        }

        self.generate_pdf(&items, output_pdf)
    }

    /// Generates a PDF with labels from a JSON file
    ///
    /// # Arguments
    /// * `json_file` - Path to JSON file with warehouse item data (array of `WarehouseItem`)
    /// * `output_pdf` - Path where the PDF file will be saved
    pub fn generate_pdf_from_json(&self, json_file: &str, output_pdf: &str) -> Result<()> {
        let content = std::fs::read_to_string(json_file)?;
        let items: Vec<WarehouseItem> = serde_json::from_str(&content)?;
        self.generate_pdf(&items, output_pdf)
    }

    /// Generates a PDF with labels for the provided items
    ///
    /// # Arguments
    /// * `items` - Slice of warehouse items to generate labels for
    /// * `output_file` - Path where the PDF file will be saved
    pub fn generate_pdf(&self, items: &[WarehouseItem], output_file: &str) -> Result<()> {
        // Create PDF document (A4: 210mm x 297mm)
        let (doc, page1, layer1) = PdfDocument::new("Warehouse Labels", Mm(210.0), Mm(297.0), "Layer 1");
        let mut current_page = doc.get_page(page1);
        let mut current_layer = current_page.get_layer(layer1);

        let labels_per_page = self.layout.labels_per_row * self.layout.rows_per_page;
        
        // Calculate total pages needed
        let total_pages = (items.len() + labels_per_page - 1) / labels_per_page;
        
        // Calculate label dimensions
        let page_width = 210.0; // A4 width in mm
        let page_height = 297.0; // A4 height in mm
        let margin = 10.0; // margin in mm
        let usable_width = page_width - (2.0 * margin);
        let usable_height = page_height - (2.0 * margin);
        
        // Additional spacing between rows to avoid overlaps
        // Must be sufficient for 3 lines of text under each QR (approximately 14mm + margin)
        let row_spacing = 4.0; // mm of extra space between rows (reduced by 2mm)
        let total_row_spacing = row_spacing * (self.layout.rows_per_page - 1) as f32;
        
        let label_width = usable_width / self.layout.labels_per_row as f32;
        let label_height = (usable_height - total_row_spacing) / self.layout.rows_per_page as f32;

        // Font for text
        let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
        let font_normal = doc.add_builtin_font(BuiltinFont::Helvetica)?;
        
        // Add footer to first page
        self.draw_page_footer(&mut current_layer, &font_normal, 1, total_pages, page_width, page_height)?;

        for (idx, item) in items.iter().enumerate() {
            // New page if needed
            if idx > 0 && idx % labels_per_page == 0 {
                let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                current_page = doc.get_page(page);
                current_layer = current_page.get_layer(layer);
                
                // Add footer to new page
                let current_page_number = (idx / labels_per_page) + 1;
                self.draw_page_footer(&mut current_layer, &font_normal, current_page_number, total_pages, page_width, page_height)?;
            }

            // Calculate label position
            let label_idx_in_page = idx % labels_per_page;
            let row = label_idx_in_page / self.layout.labels_per_row;
            let col = label_idx_in_page % self.layout.labels_per_row;
            
            // X and Y position (from top left)
            // Include extra space between rows
            let x = margin + (col as f32 * label_width);
            let y = page_height - margin - (row as f32 * (label_height + row_spacing));

            // Generate and draw QR code
            self.draw_qr_code(&mut current_layer, item, x, y, label_width, label_height)?;

            // Draw text
            self.draw_label_text(&mut current_layer, &font, &font_normal, item, x, y, label_width, label_height)?;
        }

        // Save the PDF
        doc.save(&mut BufWriter::new(File::create(output_file)?))?;

        println!("✓ PDF generated successfully: {}", output_file);
        println!("  Total labels: {}", items.len());
        println!("  Layout: {}x{} labels per page", self.layout.labels_per_row, self.layout.rows_per_page);

        Ok(())
    }

    /// Draws the QR code in the PDF
    fn draw_qr_code(
        &self,
        layer: &mut PdfLayerReference,
        item: &WarehouseItem,
        x: f32,
        y: f32,
        label_width: f32,
        _label_height: f32,
    ) -> Result<()> {
        // Generate QR code content based on selected mode
        let qr_text = match self.layout.qr_content_mode {
            QrContentMode::CodeOnly => item.code.clone(),
            QrContentMode::CodeAndLocation => item.to_qr_text_compact(),
            QrContentMode::FullJson => item.to_qr_text(),
        };
        let code = QrCode::with_error_correction_level(&qr_text, self.layout.qr_error_correction)?;
        
        let width = code.width();
        let scale = 10; // Scale to make image larger
        let size = width * scale;
        
        let mut img = ::image::ImageBuffer::new(size as u32, size as u32);
        
        for (px, py, pixel) in img.enumerate_pixels_mut() {
            let qr_x = px as usize / scale;
            let qr_y = py as usize / scale;
            let color = if code[(qr_x, qr_y)] == qrcode::Color::Dark {
                ::image::Luma([0u8])
            } else {
                ::image::Luma([255u8])
            };
            *pixel = color;
        }

        // Convert image to RGB format for printpdf
        let rgb_img = ::image::DynamicImage::ImageLuma8(img).to_rgb8();
        let (img_width, img_height) = rgb_img.dimensions();
        
        // Create image object for PDF
        let image = Image {
            image: ImageXObject {
                width: Px(img_width as usize),
                height: Px(img_height as usize),
                color_space: ColorSpace::Rgb,
                bits_per_component: ColorBits::Bit8,
                interpolate: true,
                image_data: rgb_img.into_raw(),
                image_filter: None,
                clipping_bbox: None,
                smask: None,
            },
        };

        // Calculate centered QR code position
        let qr_x = x + (label_width - self.layout.qr_size_mm) / 2.0;
        let qr_y = y - self.layout.qr_size_mm - 5.0; // 5mm from top of label

        // Calculate scale to make QR code the correct size
        // Convert QR dimensions from pixels to mm
        let qr_width_mm = img_width as f32 * 25.4 / 300.0; // Assuming 300 DPI
        let scale_factor = self.layout.qr_size_mm / qr_width_mm;

        // Draw the image
        image.add_to_layer(
            layer.clone(),
            ImageTransform {
                translate_x: Some(Mm(qr_x).into()),
                translate_y: Some(Mm(qr_y).into()),
                scale_x: Some(scale_factor),
                scale_y: Some(scale_factor),
                ..Default::default()
            },
        );

        Ok(())
    }

    /// Draws the label text
    fn draw_label_text(
        &self,
        layer: &mut PdfLayerReference,
        font_bold: &IndirectFontRef,
        font_normal: &IndirectFontRef,
        item: &WarehouseItem,
        x: f32,
        y: f32,
        label_width: f32,
        _label_height: f32,
    ) -> Result<()> {
        // Y position for text (under QR code)
        let text_start_y = y - self.layout.qr_size_mm - 10.0;

        // Horizontal center of label (QR is centered in label)
        let qr_center_x = x + (label_width / 2.0);

        // Article code (bold, 12pt) - centered relative to QR code center
        let code_y = text_start_y - 5.0;
        let code_width = self.estimate_text_width(&item.code, 12.0);
        let code_x = qr_center_x - (code_width / 2.0);
        layer.use_text(&item.code, 12.0, Mm(code_x), Mm(code_y), font_bold);

        // Description (normal, 10pt) - centered relative to QR code center
        let desc_y = code_y - 5.0;
        let truncated_desc = self.truncate_text(&item.description, 30);
        let desc_width = self.estimate_text_width(&truncated_desc, 10.0);
        let desc_x = qr_center_x - (desc_width / 2.0);
        layer.use_text(&truncated_desc, 10.0, Mm(desc_x), Mm(desc_y), font_normal);

        // Location (normal, 9pt) - centered relative to QR code center
        let loc_y = desc_y - 4.0;
        let location_text = format!("{} - {}", item.warehouse_location, item.formatted_shelf_position());
        let loc_width = self.estimate_text_width(&location_text, 9.0);
        let loc_x = qr_center_x - (loc_width / 2.0);
        layer.use_text(&location_text, 9.0, Mm(loc_x), Mm(loc_y), font_normal);

        Ok(())
    }

    /// Draws page footer with page number
    fn draw_page_footer(
        &self,
        layer: &mut PdfLayerReference,
        font: &IndirectFontRef,
        current_page: usize,
        total_pages: usize,
        page_width: f32,
        _page_height: f32,
    ) -> Result<()> {
        // Footer text: "Pagina X di Y"
        let footer_text = format!("Pagina {} di {}", current_page, total_pages);
        let footer_font_size = 8.0;
        
        // Calculate footer position (centered at bottom, 5mm from bottom)
        let footer_y = 5.0;
        let text_width = self.estimate_text_width(&footer_text, footer_font_size);
        let footer_x = (page_width - text_width) / 2.0;
        
        layer.use_text(&footer_text, footer_font_size, Mm(footer_x), Mm(footer_y), font);
        
        Ok(())
    }

    /// Approximate estimate of text width in mm
    fn estimate_text_width(&self, text: &str, font_size: f32) -> f32 {
        // Width coefficients for Helvetica (proportional to height)
        // Based on real Helvetica font metrics
        let mut total_width = 0.0;
        
        for ch in text.chars() {
            let char_width_ratio = match ch {
                // Narrow characters
                'i' | 'j' | 'l' | 'I' | '!' | '.' | ':' | ';' | ',' => 0.35,
                // Medium-narrow characters
                'f' | 't' | 'r' | '(' | ')' | '[' | ']' | '{' | '}' => 0.45,
                // Medium characters
                'a' | 'b' | 'c' | 'd' | 'e' | 'g' | 'h' | 'k' | 'n' | 'o' | 'p' | 'q' | 's' | 'u' | 'v' | 'x' | 'y' | 'z' |
                'A' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'P' | 'R' | 'T' | 'U' | 'V' | 'Y' | 'Z' |
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' | '=' | '+' | '*' | '/' | '\\' => 0.52,
                // Wide characters
                'B' | 'M' | 'N' | 'O' | 'Q' | 'S' | 'w' | 'W' | 'X' |
                '@' | '#' | '$' | '%' | '&' | '?' | '"' | '\'' => 0.58,
                // Space and others
                ' ' => 0.25,
                _ => 0.50, // default for unknown characters
            };
            
            let char_width_pt = font_size * char_width_ratio;
            let char_width_mm = char_width_pt * 0.3528; // Conversion from points to mm
            total_width += char_width_mm;
        }
        
        total_width
    }

    /// Generates an HTML file for those who prefer HTML format
    ///
    /// The HTML file can be opened in a browser and printed to PDF.
    ///
    /// # Arguments
    /// * `items` - Slice of warehouse items to generate labels for
    /// * `output_file` - Path where the HTML file will be saved
    pub fn generate_html(&self, items: &[WarehouseItem], output_file: &str) -> Result<()> {
        // Create a directory for QR images
        let qr_dir = "temp_qr_images";
        std::fs::create_dir_all(qr_dir)?;

        let mut html = String::new();
        
        // HTML header with CSS
        html.push_str(&self.generate_html_header());

        // Body
        html.push_str("<body>\n");
        html.push_str("<div class=\"page\">\n");

        let labels_per_page = self.layout.labels_per_row * self.layout.rows_per_page;

        for (idx, item) in items.iter().enumerate() {
            // New page if needed
            if idx > 0 && idx % labels_per_page == 0 {
                html.push_str("</div>\n");
                html.push_str("<div class=\"page-break\"></div>\n");
                html.push_str("<div class=\"page\">\n");
            }

            // Generate QR code and save as image
            let qr_filename = format!("{}/qr_{}_{}.png", qr_dir, item.code, idx);
            self.generate_qr_image(item, &qr_filename)?;

            // Generate HTML for the label
            html.push_str(&self.generate_label_html(item, &qr_filename));
        }

        html.push_str("</div>\n"); // Closes last page
        html.push_str("</body>\n</html>");

        // Write HTML file
        let mut file = std::fs::File::create(output_file)
            .with_context(|| format!("Unable to create file: {}", output_file))?;
        file.write_all(html.as_bytes())?;

        println!("✓ HTML file generated: {}", output_file);
        println!("  To convert to PDF:");
        println!("  1. Open in a browser and use File > Print > Save as PDF");
        println!("  2. QR images are in: {}/", qr_dir);

        Ok(())
    }

    /// Generates HTML header with CSS
    fn generate_html_header(&self) -> String {
        let label_width_pct = 100.0 / self.layout.labels_per_row as f32;
        
        format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Warehouse Labels</title>
    <style>
        @media print {{
            body {{
                margin: 0;
                padding: 0;
            }}
            .page-break {{
                page-break-after: always;
            }}
            @page {{
                size: A4;
                margin: 10mm;
            }}
        }}
        
        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}
        
        body {{
            font-family: Arial, sans-serif;
            background: #f0f0f0;
        }}
        
        .page {{
            width: 210mm;
            min-height: 297mm;
            background: white;
            margin: 0 auto 10mm auto;
            padding: 10mm;
            display: flex;
            flex-wrap: wrap;
            justify-content: flex-start;
            align-content: flex-start;
        }}
        
        .label {{
            width: {}%;
            height: calc(297mm / {});
            padding: 3mm;
            text-align: center;
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: center;
            border: 1px dashed #ccc;
        }}
        
        .qr-code {{
            width: {}mm;
            height: {}mm;
            margin-bottom: 2mm;
        }}
        
        .qr-code img {{
            width: 100%;
            height: 100%;
            object-fit: contain;
        }}
        
        .code {{
            font-size: 12pt;
            font-weight: bold;
            margin-bottom: 1mm;
            color: #000;
        }}
        
        .description {{
            font-size: 10pt;
            margin-bottom: 1mm;
            color: #333;
            max-width: 90%;
            word-wrap: break-word;
        }}
        
        .location {{
            font-size: 9pt;
            color: #666;
        }}
    </style>
</head>
"#, label_width_pct, self.layout.rows_per_page, self.layout.qr_size_mm, self.layout.qr_size_mm)
    }

    /// Generates HTML for a single label
    fn generate_label_html(&self, item: &WarehouseItem, qr_image_path: &str) -> String {
        format!(r#"    <div class="label">
        <div class="qr-code">
            <img src="{}" alt="QR Code {}" />
        </div>
        <div class="code">{}</div>
        <div class="description">{}</div>
        <div class="location">{} - {}</div>
    </div>
"#,
            qr_image_path,
            item.code,
            html_escape(&item.code),
            html_escape(&self.truncate_text(&item.description, 30)),
            html_escape(&item.warehouse_location),
            html_escape(&item.formatted_shelf_position())
        )
    }

    /// Generates QR code image and saves it to file
    fn generate_qr_image(&self, item: &WarehouseItem, output_path: &str) -> Result<()> {
        // Generate QR code content based on selected mode
        let qr_text = match self.layout.qr_content_mode {
            QrContentMode::CodeOnly => item.code.clone(),
            QrContentMode::CodeAndLocation => item.to_qr_text_compact(),
            QrContentMode::FullJson => item.to_qr_text(),
        };
        let code = QrCode::with_error_correction_level(&qr_text, self.layout.qr_error_correction)?;
        
        let width = code.width();
        let scale = 10; // Scale to make image larger
        let size = width * scale;
        
        let mut img = ::image::ImageBuffer::new(size as u32, size as u32);
        
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let qr_x = x as usize / scale;
            let qr_y = y as usize / scale;
            let color = if code[(qr_x, qr_y)] == qrcode::Color::Dark {
                ::image::Luma([0u8])
            } else {
                ::image::Luma([255u8])
            };
            *pixel = color;
        }
        
        img.save(output_path)
            .with_context(|| format!("Unable to save QR image: {}", output_path))?;
        
        Ok(())
    }

    /// Truncates text if too long
    fn truncate_text(&self, text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }
}

impl Default for PdfLabelGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Escapes HTML for security
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
