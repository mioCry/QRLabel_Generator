# Guida Completa: Generazione PDF Etichette Magazzino

## Panoramica

Questa guida spiega come utilizzare la nuova funzionalità di generazione PDF per etichette di magazzino nel progetto `qr-label-generator`.

## Caratteristiche

✅ **Generazione automatica di PDF nativi** con etichette A4  
✅ **QR Code integrati** con tutti i dati dell'articolo  
✅ **Layout personalizzabili** (3x7, 2x5, 4x10, etc.)  
✅ **Export PDF nativo** (nessuna conversione necessaria!)  
✅ **Export HTML opzionale** per personalizzazione avanzata  
✅ **Supporto CSV e JSON** come sorgenti dati  

## Struttura Etichetta

Ogni etichetta include:
1. **QR Code** (in alto) - Contiene JSON con tutti i dati
2. **Codice Articolo** (grassetto) - Es: ART-0001
3. **Descrizione** - Es: Phillips screwdriver
4. **Posizione Magazzino** - Es: MAG-A - S001-P001

## Formato Dati

### CSV
```csv
code,description,warehouse_location,shelf,shelf_relative_position,size
ART-0001,Phillips screwdriver,MAG-A,1,1,220
ART-0002,Hammer 500g,MAG-A,2,8,200
ART-0003,Adjustable wrench,MAG-A,3,5,210
```

### JSON
```json
[
  {
    "code": "ART-0001",
    "description": "Phillips screwdriver",
    "warehouse_location": "MAG-A",
    "shelf": 1,
    "shelf_relative_position": 1
  }
]
```

## Utilizzo Base

### 1. Layout Predefinito (3x7 = 21 etichette)

```rust
use qr_label_generator::PdfLabelGenerator;

fn main() -> anyhow::Result<()> {
    let generator = PdfLabelGenerator::new();
    generator.generate_pdf_from_csv(
        "warehouse_items.csv",
        "etichette.pdf"  // Genera direttamente un PDF!
    )?;
    Ok(())
}
```

### 2. Layout Personalizzato

```rust
use qr_label_generator::{PdfLabelGenerator, LabelLayout};

fn main() -> anyhow::Result<()> {
    // Layout compatto: 4x10 = 40 etichette per pagina
    let layout = LabelLayout {
        labels_per_row: 4,
        rows_per_page: 10,
        qr_size_mm: 25.0,  // QR code più piccolo (in mm)
    };
    
    let generator = PdfLabelGenerator::with_layout(layout);
    generator.generate_pdf_from_csv(
        "warehouse_items.csv",
        "etichette_compatte.pdf"
    )?;
    Ok(())
}
```

## Layout Disponibili

| Layout | Etichette/Pagina | QR Size | Descrizione |
|--------|------------------|---------|-------------|
| 3x7    | 21              | 40mm    | **Predefinito** - Ottimo equilibrio |
| 2x5    | 10              | 50mm    | QR grandi, facile lettura |
| 4x10   | 40              | 25mm    | Compatto, massima densità |

### Layout Personalizzati

```rust
let layout = LabelLayout {
    labels_per_row: 2,     // 2 colonne
    rows_per_page: 8,      // 8 righe = 16 etichette
    qr_size_mm: 35.0,      // QR da 35mm
};
```

## Generazione PDF

Il sistema ora genera **PDF nativi direttamente**, senza bisogno di conversione!

```rust
// Genera PDF direttamente
generator.generate_pdf_from_csv("items.csv", "output.pdf")?;
```

### Generazione HTML (Opzionale)

Se preferisci avere HTML per personalizzazione avanzata:

```rust
// Genera HTML con CSS print-ready
generator.generate_html(&items, "output.html")?;
```

L'HTML può essere aperto nel browser e stampato come PDF usando la funzione di stampa (Ctrl+P / Cmd+P).

## Esempio Completo

```bash
# 1. Esegui l'esempio incluso
cargo run --example warehouse_pdf

# 2. Visualizza i file PDF generati
ls -lh warehouse_labels*.pdf

# 3. Apri i PDF direttamente
open warehouse_labels_default.pdf  # Mac
xdg-open warehouse_labels_default.pdf  # Linux
start warehouse_labels_default.pdf  # Windows

# 4. I file PDF sono completi e pronti per la stampa!
```

## API Completa

### PdfLabelGenerator

```rust
// Costruttori
PdfLabelGenerator::new()  // Layout predefinito
PdfLabelGenerator::with_layout(layout)  // Layout custom

// Metodi di generazione PDF
generate_pdf_from_csv(csv_file, output_pdf)   // Da CSV a PDF
generate_pdf_from_json(json_file, output_pdf) // Da JSON a PDF
generate_pdf(items: &[WarehouseItem], output_pdf) // Da array a PDF

// Metodo opzionale per HTML
generate_html(items: &[WarehouseItem], output_html) // Genera HTML
```

### LabelLayout

```rust
pub struct LabelLayout {
    pub labels_per_row: usize,    // Colonne
    pub rows_per_page: usize,     // Righe
    pub qr_size_mm: f32,          // Dimensione QR in mm
}
```

## Contenuto del QR Code

Il QR code contiene un JSON con tutti i dati:

```json
{
  "code": "ART-0001",
  "description": "Phillips screwdriver",
  "warehouse_location": "MAG-A",
  "shelf_position": "S001-P001"
}
```

Questo può essere scansionato con qualsiasi app QR per recuperare tutte le informazioni.

## Risoluzione Problemi

### Il PDF sembra troppo grande
- I PDF contengono immagini QR in formato non compresso
- Questo è normale per preservare la qualità di scansione
- Un file da 14MB per 25 etichette è accettabile per la stampa

### Le etichette sono troppo piccole/grandi
- Modifica `qr_size_mm` nel `LabelLayout` (usa valori float come `40.0`)
- Cambia il numero di righe/colonne per aumentare/ridurre la densità

### Il testo non è centrato perfettamente
- Il sistema usa una stima della larghezza del testo
- Per layout perfetti, considera di usare `generate_html()` e personalizzare il CSS

## File di Output

Dopo l'esecuzione, troverai:
- `warehouse_labels_default.pdf` - Layout 3x7 standard (PDF nativo)
- `warehouse_labels_custom.pdf` - Layout personalizzato (PDF nativo)
- `warehouse_labels_compact.pdf` - Layout compatto 4x10 (PDF nativo)
- `warehouse_labels_from_json.pdf` - Generato da file JSON

## Note

- I file `.pdf` generati sono **veri PDF nativi**, pronti per la stampa
- Le immagini QR sono embedded direttamente nel PDF
- Non servono file esterni o conversioni aggiuntive
- Il formato HTML è ancora disponibile tramite `generate_html()` per personalizzazioni avanzate

## Performance

- Generazione: ~10-20ms per etichetta (incluso QR code)
- File PDF: ~500KB-600KB per etichetta (qualità alta per stampa)
- 25 articoli = 2 pagine A4 = ~14MB totali

## Completato ✅

- [x] Export diretto in PDF nativo
- [x] Generazione QR code embedded nel PDF
- [x] Layout multipli personalizzabili
- [x] Supporto CSV e JSON

## Prossimi Sviluppi

- [ ] Compressione delle immagini QR per ridurre dimensione file
- [ ] Supporto per codici a barre (oltre ai QR)
- [ ] Template personalizzabili con logo aziendale
- [ ] Supporto per formati etichette standard (Avery, etc.)

---

Per domande o problemi, apri una issue su GitHub!

