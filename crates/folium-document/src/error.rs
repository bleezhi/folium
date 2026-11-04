use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error("document dimensions must be greater than zero")]
    InvalidDimensions,
    #[error("unsupported Folium project version: {0}")]
    UnsupportedVersion(u32),
    #[error("project serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("project I/O failed: {0}")]
    Io(#[from] std::io::Error),
}
