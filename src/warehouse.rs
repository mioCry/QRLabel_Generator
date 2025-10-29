use serde::{Deserialize, Serialize};

/// Warehouse item structure for label generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseItem {
    /// Unique item code/identifier
    pub code: String,
    /// Item description
    pub description: String,
    /// Warehouse location identifier
    pub warehouse_location: String,
    /// Shelf number
    pub shelf: u32,
    /// Position on the shelf
    pub shelf_relative_position: u32,
}

impl WarehouseItem {
    /// Returns formatted shelf position string (e.g., "S001-P001")
    pub fn formatted_shelf_position(&self) -> String {
        format!("S{:03}-P{:03}", self.shelf, self.shelf_relative_position)
    }

    /// Returns a complete JSON text payload to insert into the QR code
    pub fn to_qr_text(&self) -> String {
        #[derive(Serialize)]
        struct Payload<'a> {
            code: &'a str,
            description: &'a str,
            warehouse_location: &'a str,
            shelf_position: String,
        }

        let payload = Payload {
            code: &self.code,
            description: &self.description,
            warehouse_location: &self.warehouse_location,
            shelf_position: self.formatted_shelf_position(),
        };

        serde_json::to_string(&payload).unwrap_or_default()
    }

    /// Returns a compact payload (code + location) for less dense QR codes
    pub fn to_qr_text_compact(&self) -> String {
        format!("{}|{}|{}", 
            self.code, 
            self.warehouse_location, 
            self.formatted_shelf_position()
        )
    }
}


