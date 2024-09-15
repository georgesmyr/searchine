use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::read_to_string;

/// Structure that represents a document. Document in this
/// case is any part of ++++++
pub struct Document {
    doc_id: u32,
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
    pub fn new(doc_id: u32, page_content: String, metadata: DocumentMetadata) -> Self {
        Self { doc_id, page_content, metadata }
    }

    /// Loads a document from file.
    pub fn from_file(doc_id: u32, path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = read_to_string(&path)
            .context(format!("Failed to read file {}", path.display()))?;
        let metadata = DocumentMetadata::new(DocumentSource::File(path));
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

#[derive(Debug, Eq, PartialEq)]
pub enum DocumentSource {
    File(PathBuf),
}

/// Structure that stores metadata for a document.
/// The metadata could be the document source, the source
/// type, etc.
pub struct DocumentMetadata {
    source: DocumentSource,
}

impl DocumentMetadata {
    pub fn new(source: DocumentSource) -> Self {
        DocumentMetadata { source }
    }
}

