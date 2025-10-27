use qr_label_generator::QrGenerator;

fn main() -> anyhow::Result<()> {
    let gen = QrGenerator::new();
    gen.generate_batch_from_csv("./examples/qr_layout.csv", "./output_test")
}


