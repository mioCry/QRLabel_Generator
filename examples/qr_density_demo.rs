use qr_label_generator::{PdfLabelGenerator, LabelLayout, QrContentMode};
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 Demo of different QR Code densities\n");
    
    // 1. QR Code with article code only (minimum density - easy to scan)
    println!("1. Generating PDF with QR CODE ARTICLE CODE ONLY (minimum density)");
    let layout_code_only = LabelLayout {
        labels_per_row: 3,
        rows_per_page: 4,
        qr_size_mm: 40.0,
        qr_content_mode: QrContentMode::CodeOnly,
        qr_error_correction: qrcode::EcLevel::L, // Low - optimal for simple QR codes
    };
    
    let generator = PdfLabelGenerator::with_layout(layout_code_only);
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels_qr_code_only.pdf"
    )?;
    println!("   ✓ PDF generated: warehouse_labels_qr_code_only.pdf");
    println!("   • Contains: article code only (e.g. 'ART-0001')");
    println!("   • Density: MINIMUM - very easy to scan\n");

    // 2. Compact QR Code with code + location (medium density)
    println!("2. Generating PDF with COMPACT QR CODE (medium density)");
    let layout_compact = LabelLayout {
        labels_per_row: 3,
        rows_per_page: 4,
        qr_size_mm: 40.0,
        qr_content_mode: QrContentMode::CodeAndLocation,
        qr_error_correction: qrcode::EcLevel::M, // Medium - good compromise
    };
    
    let generator = PdfLabelGenerator::with_layout(layout_compact);
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels_qr_compact.pdf"
    )?;
    println!("   ✓ PDF generated: warehouse_labels_qr_compact.pdf");
    println!("   • Contains: code|location|position (e.g. 'ART-0001|MAG-A|S001-P001')");
    println!("   • Density: MEDIUM - easy to scan\n");

    // 3. Complete QR Code with all JSON data (maximum density)
    println!("3. Generating PDF with COMPLETE QR CODE JSON (maximum density)");
    let layout_full = LabelLayout {
        labels_per_row: 3,
        rows_per_page: 4,
        qr_size_mm: 40.0,
        qr_content_mode: QrContentMode::FullJson,
        qr_error_correction: qrcode::EcLevel::H, // High - necessary for lots of data
    };
    
    let generator = PdfLabelGenerator::with_layout(layout_full);
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels_qr_full.pdf"
    )?;
    println!("   ✓ PDF generated: warehouse_labels_qr_full.pdf");
    println!("   • Contains: complete JSON with all details");
    println!("   • Density: MAXIMUM - harder to scan\n");

    println!("📊 Summary:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("│ Mode              │ QR Content             │ Scan Ease    │");
    println!("│━━━━━━━━━━━━━━━━━━━│━━━━━━━━━━━━━━━━━━━━━━━━│━━━━━━━━━━━━━━━│");
    println!("│ CodeOnly          │ Code only              │ ⭐⭐⭐⭐⭐        │");
    println!("│ CodeAndLocation   │ Code + location        │ ⭐⭐⭐⭐          │");
    println!("│ FullJson          │ All JSON details       │ ⭐⭐⭐            │");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n💡 Tip: Use CodeOnly for maximum scan ease!");
    println!("   If you need more information, use CodeAndLocation.");

    Ok(())
}

