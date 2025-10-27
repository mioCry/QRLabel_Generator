use qr_label_generator::{PdfLabelGenerator, LabelLayout, QrContentMode};

fn main() -> anyhow::Result<()> {
    // Example 1: Generate PDF from CSV with default layout (3x7 = 21 labels per page)
    println!("Generating PDF from CSV with default layout...");
    let generator = PdfLabelGenerator::new();
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels_default.pdf"
    )?;
    println!("✓ PDF generated: warehouse_labels_default.pdf");

    // Example 2: Generate PDF from JSON with default layout
    println!("\nGenerating PDF from JSON with default layout...");
    generator.generate_pdf_from_json(
        "examples/warehouse_items.json",
        "warehouse_labels_from_json.pdf"
    )?;
    println!("✓ PDF generated: warehouse_labels_from_json.pdf");

    // Example 3: Custom layout (2x3 = 6 labels per page, larger QR code)
    println!("\nGenerating PDF with custom layout (2x3)...");
    let custom_layout = LabelLayout {
        labels_per_row: 2,
        rows_per_page: 3,
        qr_size_mm: 50.0,
        qr_content_mode: QrContentMode::CodeAndLocation,
        qr_error_correction: qrcode::EcLevel::M,
    };
    let custom_generator = PdfLabelGenerator::with_layout(custom_layout);
    custom_generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels_custom.pdf"
    )?;
    println!("✓ PDF generated: warehouse_labels_custom.pdf");

    // Example 4: Compact layout (4x5 = 20 labels per page, small QR code)
    println!("\nGenerating PDF with compact layout (4x5)...");
    let compact_layout = LabelLayout {
        labels_per_row: 4,
        rows_per_page: 5,
        qr_size_mm: 25.0,
        qr_content_mode: QrContentMode::CodeOnly,
        qr_error_correction: qrcode::EcLevel::L,
    };
    let compact_generator = PdfLabelGenerator::with_layout(compact_layout);
    compact_generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels_compact.pdf"
    )?;
    println!("✓ PDF generated: warehouse_labels_compact.pdf");

    println!("\n✓ All PDF files have been generated successfully!");
    println!("\nSummary:");
    println!("  - warehouse_labels_default.pdf: Standard 3x7 layout (25 labels on 2 pages)");
    println!("  - warehouse_labels_from_json.pdf: From JSON file (2 labels on 1 page)");
    println!("  - warehouse_labels_custom.pdf: Custom 2x3 layout (25 labels on 5 pages)");
    println!("  - warehouse_labels_compact.pdf: Compact 4x5 layout (25 labels on 2 pages)");
    println!("\nYou can open the PDFs with any PDF viewer!");

    Ok(())
}

