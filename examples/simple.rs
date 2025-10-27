use qr_label_generator::{Config, QrGenerator};

fn main() -> anyhow::Result<()> {
    // Use default configuration and generate a simple QR code
    let config = Config::default();
    let generator = QrGenerator::from_config(&config);
    generator.generate_single("https://www.rust-lang.org", "./output_test/rust_website.png", 220)?;
    Ok(())
}


