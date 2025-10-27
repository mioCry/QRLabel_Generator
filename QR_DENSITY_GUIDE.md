# QR Code Density Guide

## 📋 Overview

The QR label generator now supports **three density modes** for QR codes, allowing you to choose the best compromise between amount of information and ease of scanning.

## 🎯 Available Modes

### 1. **CodeOnly** (Minimum Density) ⭐⭐⭐⭐⭐

**Content:** Article code only (e.g. `ART-0001`)

**Advantages:**
- Very simple and fast to scan QR code
- Works even with cheap scanners or in poor lighting conditions
- Perfect for rapid item identification
- Lower chance of reading errors

**When to use it:**
- Inventory system where database is always accessible
- Need for fast and reliable scanning
- Environments with non-optimal lighting
- Medium/low quality QR scanners

**Recommended error correction level:** `L` (Low)

### 2. **CodeAndLocation** (Medium Density) ⭐⭐⭐⭐

**Content:** Code + location + shelf position (e.g. `ART-0001|MAG-A|S001-P001`)

**Advantages:**
- Good compromise between information and scannability
- Includes essential location information
- Can work offline for basic operations
- Still relatively easy to scan

**When to use it:**
- Need for immediate location information
- Operations that might occur offline
- Hybrid system with database + local info
- Quick verification of item position

**Recommended error correction level:** `M` (Medium)

### 3. **FullJson** (Maximum Density) ⭐⭐⭐

**Content:** Complete JSON with all item details

```json
{
  "code": "ART-0001",
  "description": "Phillips screwdriver",
  "warehouse_location": "MAG-A",
  "shelf_position": "S001-P001"
}
```

**Advantages:**
- All item information in the QR code
- Completely offline operation
- Self-contained, no database required

**Disadvantages:**
- Denser and more complex QR code
- Requires good quality scanners
- Slower to scan
- More sensitive to damage or dirt

**When to use it:**
- Completely offline system
- Need all information on the label
- High quality scanners available
- Optimal scanning conditions

**Recommended error correction level:** `H` (High)

## 🔧 Error Correction Levels

QR codes can include redundant data to correct reading errors:

| Level | Redundancy | When to use it |
|-------|-----------|---------------|
| **L** (Low) | ~7% | Simple QR, optimal conditions |
| **M** (Medium) | ~15% | General use, good compromise |
| **Q** (Quartile) | ~25% | Labels subject to wear |
| **H** (High) | ~30% | Dense QR, difficult conditions |

## 💻 Usage Examples

### Example 1: Minimalist QR (Recommended for most cases)

```rust
use qr_label_generator::{PdfLabelGenerator, LabelLayout, QrContentMode};

let layout = LabelLayout {
    labels_per_row: 3,
    rows_per_page: 4,
    qr_size_mm: 40.0,
    qr_content_mode: QrContentMode::CodeOnly,
    qr_error_correction: qrcode::EcLevel::L,
};

let generator = PdfLabelGenerator::with_layout(layout);
generator.generate_pdf_from_csv(
    "warehouse_items.csv",
    "labels_simple.pdf"
)?;
```

### Example 2: QR with Location

```rust
let layout = LabelLayout {
    labels_per_row: 3,
    rows_per_page: 4,
    qr_size_mm: 40.0,
    qr_content_mode: QrContentMode::CodeAndLocation,
    qr_error_correction: qrcode::EcLevel::M,
};

let generator = PdfLabelGenerator::with_layout(layout);
generator.generate_pdf_from_csv(
    "warehouse_items.csv",
    "labels_with_location.pdf"
)?;
```

### Example 3: Complete QR

```rust
let layout = LabelLayout {
    labels_per_row: 3,
    rows_per_page: 4,
    qr_size_mm: 40.0,
    qr_content_mode: QrContentMode::FullJson,
    qr_error_correction: qrcode::EcLevel::H,
};

let generator = PdfLabelGenerator::with_layout(layout);
generator.generate_pdf_from_csv(
    "warehouse_items.csv",
    "labels_full.pdf"
)?;
```

### Example 4: Using Default Layout

```rust
// Automatically uses CodeOnly with EcLevel::L
let generator = PdfLabelGenerator::new();
generator.generate_pdf_from_csv(
    "warehouse_items.csv",
    "labels_default.pdf"
)?;
```

## 🧪 Density Demo

To test all three modes and compare them:

```bash
cargo run --release --example qr_density_demo
```

This will generate three PDFs:
- `warehouse_labels_qr_code_only.pdf` - Simple QR
- `warehouse_labels_qr_compact.pdf` - QR with location
- `warehouse_labels_qr_full.pdf` - Complete QR

## 📊 Comparison Table

| Aspect | CodeOnly | CodeAndLocation | FullJson |
|--------|----------|-----------------|----------|
| **QR Size** | Small | Medium | Large |
| **Scan ease** | Excellent | Good | Medium |
| **Info contained** | Minimal | Essential | Complete |
| **Offline use** | No | Partial | Yes |
| **Scanner required** | Basic | Medium | Good |
| **Scan speed** | Fast | Medium | Slow |
| **Damage resistance** | High | Medium | Low |

## 🎯 Recommendations

### ✅ Use CodeOnly if:
- You always have internet/database access
- You want maximum scanning speed
- You use cheap scanners or smartphone apps
- Labels can get dirty or damaged

### ✅ Use CodeAndLocation if:
- You need immediate location information
- Possible temporary offline operations
- Need a good compromise
- You have medium quality scanners

### ✅ Use FullJson if:
- You work ALWAYS offline
- You have professional scanners
- Labels are protected
- You need ALL information on the label

## 🔍 Notes on CSV "size" Field

The `size` field present in the CSV is currently read but **not used** in label generation. This field is reserved for future implementations where it might be used for:
- Automatically varying QR size based on data amount
- Filtering items by size
- Customizing layout based on item size

## 📱 Scanning Tests

After generating the PDFs, test scanning with:

1. **Smartphone** (generic QR scanner app)
   - CodeOnly: ✅ Excellent
   - CodeAndLocation: ✅ Good
   - FullJson: ⚠️ May require multiple attempts

2. **Industrial barcode scanner**
   - All modes should work well

3. **Cheap barcode scanner**
   - Prefer CodeOnly
   - Avoid FullJson

## 🆘 Common Problems

### QR code hard to scan?
- ✅ Try using `CodeOnly` instead of `FullJson`
- ✅ Increase `qr_size_mm` (e.g. from 40 to 50mm)
- ✅ Use a lower error correction level (L instead of H)
- ✅ Make sure labels are clean and well lit

### Scanner doesn't read some QR?
- ✅ Verify scanner supports QR codes (not just barcodes)
- ✅ Increase QR size
- ✅ Reduce data amount (use simpler mode)
- ✅ Print with higher quality (300 DPI minimum)

## 📄 Generated Files

Running the examples will create the following PDFs:
- `warehouse_labels_default.pdf` - Default layout (CodeOnly)
- `warehouse_labels_qr_code_only.pdf` - Code only
- `warehouse_labels_qr_compact.pdf` - Code + location
- `warehouse_labels_qr_full.pdf` - Complete JSON

Compare the QR codes visually in these files to choose the best mode for your use case!

---

**💡 Final tip:** For most uses, **`CodeOnly`** offers the best balance between simplicity, speed and reliability. Switch to more complex modes only if you have specific offline information needs.



