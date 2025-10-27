use qr_label_generator::QrGenerator;

fn main() -> anyhow::Result<()> {
    let gen = QrGenerator::new();
    gen.generate_warehouse_batch("./examples/warehouse_items.json", "./output_test", Some(220))?;
    Ok(())
}


