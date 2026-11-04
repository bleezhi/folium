//! Shared primitives used by Folium applications and engine crates.

pub mod color;
pub mod geometry;
pub mod ids;

pub use color::Rgba;
pub use geometry::{Point, Rect, Size};
pub use ids::{DocumentId, LayerId, new_document_id, new_layer_id};
