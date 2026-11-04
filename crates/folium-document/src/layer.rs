use folium_core::{LayerId, Rgba, new_layer_id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerKind {
    Raster,
    Group,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub kind: LayerKind,
    pub visible: bool,
    pub opacity: f32,
    pub background: Rgba,
}

impl Layer {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: new_layer_id(),
            name: name.into(),
            kind: LayerKind::Raster,
            visible: true,
            opacity: 1.0,
            background: Rgba::TRANSPARENT,
        }
    }
}
