use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LayerId(pub Uuid);

#[must_use]
pub fn new_document_id() -> DocumentId {
    DocumentId(Uuid::new_v4())
}

#[must_use]
pub fn new_layer_id() -> LayerId {
    LayerId(Uuid::new_v4())
}
