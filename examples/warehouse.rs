use qr_label_generator::{QrGenerator, WarehouseItem};

fn main() -> anyhow::Result<()> {
    let item = WarehouseItem {
        code: "ART-0001".to_string(),
        description: "Phillips screwdriver".to_string(),
        warehouse_location: "MAG-A".to_string(),
        shelf: 1,
        shelf_relative_position: 1,
    };

    let gen = QrGenerator::new();
    gen.generate_from_warehouse_item(&item, "./output_test/warehouse_art1.png", 220)?;
    Ok(())
}


