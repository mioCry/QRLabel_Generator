use qr_label_generator::QrGenerator;

fn main() -> anyhow::Result<()> {
    let gen = QrGenerator::new();
    gen.generate_warehouse_batch_from_csv("./examples/warehouse_items.csv", "./output_test", Some(220))
}


