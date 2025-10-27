# 🚀 Quick Start - Optimized QR Codes

## ⚡ In 30 Seconds

### 1. Generate Labels with Optimized QR Codes (Simple Method)
```bash
cargo run --release --example warehouse_pdf
```

✅ Automatically generates:
- Simple and fast to scan QR codes
- Optimized 3x4 layout (12 labels/page)
- Fully visible text
- File: `warehouse_labels_default.pdf`

### 2. Test All QR Modes
```bash
cargo run --release --example qr_density_demo
```

✅ Compare visually:
- Code-only QR (very simple) ⭐⭐⭐⭐⭐
- QR with location (medium) ⭐⭐⭐⭐
- Complete JSON QR (dense) ⭐⭐⭐

## 📋 Which Mode to Choose?

### 🥇 Use CodeOnly (DEFAULT) If:
- ✅ You have an always accessible database/system
- ✅ You want maximum scanning speed
- ✅ You use cheap scanners or smartphones
- ✅ You only need to identify the item

**It's the right choice for 95% of cases!**

### 🥈 Use CodeAndLocation If:
- ✅ You need to quickly verify location
- ✅ System can go offline temporarily
- ✅ You want a good compromise

### 🥉 Use FullJson Only If:
- ⚠️ System is ALWAYS offline
- ⚠️ You have professional scanners
- ⚠️ You need ALL content in the QR

## 💻 Minimal Code

### Basic Generation (Recommended)
```rust
use qr_label_generator::PdfLabelGenerator;

fn main() -> anyhow::Result<()> {
    let generator = PdfLabelGenerator::new();
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "my_labels.pdf"
    )?;
    Ok(())
}
```

### Complete Customization
```rust
use qr_label_generator::{PdfLabelGenerator, LabelLayout, QrContentMode};

fn main() -> anyhow::Result<()> {
    let layout = LabelLayout {
        labels_per_row: 3,
        rows_per_page: 4,
        qr_size_mm: 40.0,
        qr_content_mode: QrContentMode::CodeOnly,  // ⭐ Best choice
        qr_error_correction: qrcode::EcLevel::L,   // ⭐ Simple QR
    };
    
    let generator = PdfLabelGenerator::with_layout(layout);
    generator.generate_pdf_from_csv(
        "examples/warehouse_items.csv",
        "my_labels.pdf"
    )?;
    Ok(())
}
```

## 📁 CSV Format

Your CSV file must have these fields:
```csv
code,description,warehouse_location,shelf,shelf_relative_position,size
ART-0001,Phillips screwdriver,MAG-A,1,1,220
ART-0002,Hammer 500g,MAG-A,2,8,200
```

**Note**: The `size` field is read but not yet used.

## 🎯 Quick Comparison

| Feature | CodeOnly | CodeAndLocation | FullJson |
|---------|----------|-----------------|----------|
| **QR Content** | `ART-0001` | `ART-0001\|MAG-A\|S001-P001` | Complete JSON |
| **PDF Size** | 31KB | 40KB | 310KB |
| **Scan Speed** | ⚡⚡⚡⚡⚡ | ⚡⚡⚡⚡ | ⚡⚡⚡ |
| **Scanner Required** | Basic/Smartphone | Medium | Professional |
| **Works Offline** | ❌ | Partial | ✅ |
| **Recommended** | ✅ Yes! | If needed | Rare |

## 🔍 Verify Result

1. Open the generated PDF
2. Look at QR code density:
   - **CodeOnly**: Simple QR, few modules
   - **CodeAndLocation**: Medium QR
   - **FullJson**: Very dense QR

3. Try scanning with smartphone:
   - CodeOnly: instant scan ✅
   - FullJson: may require multiple attempts ⚠️

## 🛠️ Common Problems

### ❓ "QR doesn't scan"
**Solution**: Use `CodeOnly` instead of `FullJson`
```rust
qr_content_mode: QrContentMode::CodeOnly,  // ← Change here
```

### ❓ "Text is covered by QR below"
**Solution**: Already fixed! The default layout has optimal 6mm spacing.

### ❓ "Want more labels per page"
**Solution**: Modify `rows_per_page`, but watch out for readability:
```rust
let layout = LabelLayout {
    labels_per_row: 4,    // More columns
    rows_per_page: 6,     // More rows
    qr_size_mm: 30.0,     // Smaller QR
    qr_content_mode: QrContentMode::CodeOnly,
    qr_error_correction: qrcode::EcLevel::L,
};
```

### ❓ "Need CSV size field?"
**Answer**: Currently read but NOT used. Reserved for future implementations.

## 📚 Complete Documentation

- **Quick examples**: This file
- **Complete QR guide**: `QR_DENSITY_GUIDE.md`
- **All details**: `README.md`
- **Technical changes**: `CHANGELOG_QR_OPTIMIZATION.md`
- **Summary**: `RIEPILOGO_OTTIMIZZAZIONI.md`

## 🎓 Pro Tips

1. **Always use defaults** unless you have specific needs
2. **Test on smartphone** before printing 1000 labels
3. **Print at 300 DPI** minimum for optimal quality
4. **Keep QR clean** - dust impacts scanning

## ✅ Pre-Printing Checklist

- [ ] Tested QR on smartphone
- [ ] Verified text is visible
- [ ] Checked spelling in data
- [ ] Printed a test page
- [ ] Verified label size is correct

## 🎉 Ready to Go!

You're now ready to generate optimized QR labels! 

```bash
# Generate your labels
cargo run --release --example warehouse_pdf

# Open the PDF
open warehouse_labels_default.pdf  # macOS
# or
xdg-open warehouse_labels_default.pdf  # Linux
```

**Good luck! 🚀**

---

*Questions? Check out `QR_DENSITY_GUIDE.md` for the complete guide.*

