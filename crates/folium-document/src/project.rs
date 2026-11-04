use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{Document, DocumentError};

/// Initial JSON-backed project format. The on-disk container can evolve without
/// changing the in-memory document model.
pub const PROJECT_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectFile {
    pub format: String,
    pub version: u32,
    pub document: Document,
}

impl ProjectFile {
    #[must_use]
    pub fn new(document: Document) -> Self {
        Self {
            format: "folium".to_owned(),
            version: PROJECT_FORMAT_VERSION,
            document,
        }
    }
}

pub fn save_project(path: impl AsRef<Path>, document: &Document) -> Result<(), DocumentError> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(BufWriter::new(file), &ProjectFile::new(document.clone()))?;
    Ok(())
}

pub fn load_project(path: impl AsRef<Path>) -> Result<Document, DocumentError> {
    let file = File::open(path)?;
    let project: ProjectFile = serde_json::from_reader(BufReader::new(file))?;
    if project.format != "folium" {
        return Err(DocumentError::UnsupportedVersion(project.version));
    }
    if project.version != PROJECT_FORMAT_VERSION {
        return Err(DocumentError::UnsupportedVersion(project.version));
    }
    Ok(project.document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_round_trip() {
        let path = std::env::temp_dir().join(format!("folium-{}.folium", std::process::id()));
        let document = Document::new("Test", 1920, 1080).unwrap();
        save_project(&path, &document).unwrap();
        let restored = load_project(&path).unwrap();
        std::fs::remove_file(path).unwrap();
        assert_eq!(restored, document);
    }
}
