//! Document structures and the first version of the Folium project format.

mod document;
mod error;
mod layer;
mod project;

pub use document::Document;
pub use error::DocumentError;
pub use layer::{Layer, LayerKind};
pub use project::{PROJECT_FORMAT_VERSION, ProjectFile, load_project, save_project};
