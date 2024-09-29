use std::path::PathBuf;

/// Structure that stores metadata for a document.
/// The metadata could be the document source, the source
/// type, etc.
#[derive(Default, Debug, Clone)]
pub struct DocumentMetadata {
    source: Option<DocumentSource>,
}

#[derive(Default, Debug)]
pub struct DocumentMetadataBuilder {
    meta: DocumentMetadata,
}

impl DocumentMetadataBuilder {
    /// Creates a new builder for document's metadata, with no metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the document source.
    pub fn with_source(self, source: DocumentSource) -> Self {
        DocumentMetadataBuilder {
            meta: DocumentMetadata {
                source: Some(source),
                ..self.meta
            },
        }
    }

    /// Consumes the document metadata builder and returns the metadata.
    pub fn build(self) -> DocumentMetadata {
        self.meta
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DocumentSource {
    File(PathBuf),
}
