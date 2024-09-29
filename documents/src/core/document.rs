use std::path::Path;

use anyhow::Context;

use crate::core::metadata::{DocumentMetadata, DocumentMetadataBuilder, DocumentSource};
use crate::read_to_string;
use crate::DocumentId;

/// Structure that represents a document. Document in this
/// case is any part of ++++++
pub struct Document {
    doc_id: DocumentId,
    page_content: String,
    metadata: DocumentMetadata,
}

impl Document {
    /// Creates a new Document with specified document ID, content,
    /// and metadata.
    ///
    /// # Arguments
    ///
    /// - `doc_id`: Document ID.
    /// - `page_content`: Content of the document.
    /// - `metadata`: Metadata for the document, e.g. source, source type, etc.
    pub fn new(doc_id: DocumentId, page_content: String, metadata: DocumentMetadata) -> Self {
        Self {
            doc_id,
            page_content,
            metadata,
        }
    }

    /// Loads a document from file.
    pub fn from_file(doc_id: DocumentId, path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content =
            read_to_string(&path).context(format!("Failed to read file {}", path.display()))?;
        let metadata = DocumentMetadataBuilder::new()
            .with_source(DocumentSource::File(path))
            .build();
        Ok(Document::new(doc_id, content, metadata))
    }

    /// Returns the document ID
    pub fn doc_id(&self) -> u32 {
        self.doc_id
    }

    /// Returns the page contents of the document.
    pub fn page_content(&self) -> &str {
        &self.page_content
    }
}
