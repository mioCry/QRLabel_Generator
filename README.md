# QR Label Generator (crate)

Rust library to generate QR code images, with simple APIs for single and batch generation from JSON files. Publishable on crates.io.

## Features

- ✅ Single QR code generation
- ✅ Batch generation from JSON and CSV files
- ✅ Size customization
- ✅ Robust error handling with `anyhow`
- ✅ PNG output format (via `image`)
- ✅ **Warehouse PDF label generation** (A4 format with multi-layout)
- ✅ Support for warehouse items with shelf position
- ✅ Customizable layouts (3x4, 2x5, 4x10, etc.)
- ✅ **Three QR density modes**: CodeOnly, CodeAndLocation, FullJson
- ✅ **Configurable error correction levels** (L, M, Q, H)
- ✅ Automatic optimization for ease of scanning

## Installation

### Prerequisites

- Rust 1.70+ installed on the system
- Cargo (included with Rust)

### Adding as a dependency

In your `Cargo.toml`:

```
[dependencies]
qr-label-generator = "0.1"
```

## Usage

### Single QR code generation

```rust
use qr_label_generator::{Config, QrGenerator};

fn main() -> anyhow::Result<()> {
    let cfg = Config::default();
    let gen = QrGenerator::from_config(&cfg);
    gen.generate_single("https://example.com", "website.png", 300)?;
    Ok(())
}
```

### Batch generation

Create a JSON file with QR code data:

```json
[
  {
    "text": "https://example.com",
    "filename": "website.png",
    "size": 200
  },
  {
    "text": "mailto:info@example.com",
    "filename": "email.png",
    "size": 150
  },
  {
    "text": "tel:+1234567890",
    "filename": "phone.png"
  }
]
```

Then invoke in code:

```rust
use qr_label_generator::QrGenerator;

fn main() -> anyhow::Result<()> {
    let gen = QrGenerator::new();
    gen.generate_batch("qr_data.json", "output/")
}
```

### Configuration

The `Config` structure allows setting default size, error correction level (L/M/Q/H) and foreground/background colors.

## Project structure

```
QRLabel_Generator/
├── Cargo.toml          # Project configuration and dependencies
├── README.md           # This file
├── src/
│   ├── lib.rs          # Library API
│   ├── qr.rs           # QR generation logic
│   └── config.rs       # Application configuration
└── examples/           # Example files
    ├── sample_data.json
    └── simple.rs
```

## Dependencies

- `qrcode`: For QR code generation
- `image`: For image manipulation
- `anyhow`: For error handling
- `serde`: For JSON serialization/deserialization

## Examples

### Example: Simple QR code

```bash
cargo run --example simple
```

See also `examples/sample_data.json` to use with `QrGenerator::generate_batch`.

### QR code batch

Create the file `examples.json`:

```json
[
  {
    "text": "https://github.com",
    "filename": "github.png",
    "size": 200
  },
  {
    "text": "https://stackoverflow.com",
    "filename": "stackoverflow.png",
    "size": 200
  }
]
```

Then use `QrGenerator::generate_batch` in your code.


## Development

### Running tests

```bash
cargo test
```

### Warehouse items

Generates QR codes for items, including code, description, warehouse and shelf position in the format `S001-P001`.

```rust
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
```

### Format code

```bash
cargo fmt
```

#### Batch from JSON

You can generate labels in batch by reading a JSON file with a list of `WarehouseItem`.

```rust
use qr_label_generator::QrGenerator;

fn main() -> anyhow::Result<()> {
    let gen = QrGenerator::new();
    gen.generate_warehouse_batch("./examples/warehouse_items.json", "./output_test", Some(220))
}
```

Example file: `examples/warehouse_items.json`.

## CSV

### Generic CSV generation

CSV with headers `text,filename,size`:

```
text,filename,size
https://www.rust-lang.org,rust_website.png,220
tel:+1234567890,phone_number.png,200
```

Code:

```rust
use qr_label_generator::QrGenerator;

fn main() -> anyhow::Result<()> {
    let gen = QrGenerator::new();
    gen.generate_batch_from_csv("./examples/qr_layout.csv", "./output_test")
}
```

### Warehouse from CSV

CSV with headers `code,description,warehouse_location,shelf,shelf_relative_position,size`.

```rust
use qr_label_generator::QrGenerator;

fn main() -> anyhow::Result<()> {
    let gen = QrGenerator::new();
    gen.generate_warehouse_batch_from_csv("./examples/warehouse_items.csv", "./output_test", Some(220))
}
```

## 📄 Warehouse PDF Label Generation (NEW!)

Generates warehouse item labels in A4 format ready for printing. Each label includes:
- **QR Code** with configurable density (CodeOnly, CodeAndLocation, or FullJson)
- **Item code** (in bold)
- **Item description**
- **Warehouse location** (format: MAG-A - S001-P001)

### Default Layout (3x4 = 12 labels per page)

```rust
use qr_label_generator::PdfLabelGenerator;

fn main() -> anyhow::Result<()> {
    let generator = PdfLabelGenerator::new();
    
    // From CSV file
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels.html"
    )?;
    
    // From JSON file
    generator.generate_pdf_from_json(
        "examples/warehouse_items.json",
        "warehouse_labels.html"
    )?;
    
    Ok(())
}
```

### Custom Layout

```rust
use qr_label_generator::{PdfLabelGenerator, LabelLayout, QrContentMode};

fn main() -> anyhow::Result<()> {
    // Compact layout: 4x10 = 40 labels per page
    let compact_layout = LabelLayout {
        labels_per_row: 4,
        rows_per_page: 10,
        qr_size_mm: 25.0,
        qr_content_mode: QrContentMode::CodeOnly,
        qr_error_correction: qrcode::EcLevel::L,
    };
    
    let generator = PdfLabelGenerator::with_layout(compact_layout);
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "warehouse_labels_compact.pdf"
    )?;
    
    Ok(())
}
```

### 🎯 QR Code Density Modes (NEW!)

Choose the optimal density level for your QR codes:

#### 1. **CodeOnly** (Recommended) ⭐⭐⭐⭐⭐
```rust
let layout = LabelLayout {
    labels_per_row: 3,
    rows_per_page: 4,
    qr_size_mm: 40.0,
    qr_content_mode: QrContentMode::CodeOnly,  // Item code only
    qr_error_correction: qrcode::EcLevel::L,   // Low correction = simpler QR
};
```
- **Content**: Code only (e.g. `ART-0001`)
- **Advantages**: Fastest scanning, works with any scanner
- **Use when**: You always have database access

#### 2. **CodeAndLocation** ⭐⭐⭐⭐
```rust
let layout = LabelLayout {
    labels_per_row: 3,
    rows_per_page: 4,
    qr_size_mm: 40.0,
    qr_content_mode: QrContentMode::CodeAndLocation,  // Code + location
    qr_error_correction: qrcode::EcLevel::M,          // Medium correction
};
```
- **Content**: `ART-0001|MAG-A|S001-P001`
- **Advantages**: Includes location info, good compromise
- **Use when**: Need to verify position quickly

#### 3. **FullJson** ⭐⭐⭐
```rust
let layout = LabelLayout {
    labels_per_row: 3,
    rows_per_page: 4,
    qr_size_mm: 40.0,
    qr_content_mode: QrContentMode::FullJson,  // Complete JSON
    qr_error_correction: qrcode::EcLevel::H,   // High correction needed
};
```
- **Content**: Complete JSON with all details
- **Advantages**: Works offline, self-contained
- **Use when**: System completely offline

📖 **Complete guide**: See [QR_DENSITY_GUIDE.md](QR_DENSITY_GUIDE.md) for details and comparisons.

🧪 **Demo**: Try all modes with:
```bash
cargo run --release --example qr_density_demo
```

### Converting HTML to PDF

Generated files are print-ready HTML. To convert them to PDF:

#### Method 1: Browser (Simplest)
1. Open the HTML file in Chrome, Firefox or Safari
2. File > Print (or Ctrl+P / Cmd+P)
3. Select "Save as PDF" as destination
4. Save the PDF

#### Method 2: Command line (Automatic)

With wkhtmltopdf:
```bash
wkhtmltopdf warehouse_labels.html warehouse_labels.pdf
```

With WeasyPrint:
```bash
weasyprint warehouse_labels.html warehouse_labels.pdf
```

With Chrome headless:
```bash
chrome --headless --print-to-pdf=warehouse_labels.pdf warehouse_labels.html
```

### CSV Format for Labels

```csv
code,description,warehouse_location,shelf,shelf_relative_position,size
ART-0001,Phillips screwdriver,MAG-A,1,1,220
ART-0002,Hammer 500g,MAG-A,2,8,200
ART-0003,Adjustable wrench,MAG-A,3,5,210
```

### Complete Example

See `examples/warehouse_pdf.rs` for a complete example with multiple layouts.

```bash
cargo run --example warehouse_pdf
```

### Check code

```bash
cargo clippy
```

## License

Dual license: MIT or Apache-2.0, at your choice.

- See `LICENSE` for MIT terms
- See `LICENSE-APACHE` for Apache-2.0 terms

## Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request
