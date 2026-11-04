use folium_core::{DocumentId, Size, new_document_id};
use serde::{Deserialize, Serialize};

use crate::{DocumentError, Layer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub name: String,
    pub size: Size,
    pub layers: Vec<Layer>,
}

impl Document {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Result<Self, DocumentError> {
        let size = Size::new(width as f32, height as f32);
        if !size.is_valid() {
            return Err(DocumentError::InvalidDimensions);
        }

        Ok(Self {
            id: new_document_id(),
            name: name.into(),
            size,
            layers: vec![Layer::new("Background")],
        })
    }
}
