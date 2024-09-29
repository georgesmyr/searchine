use crate::{Document, DocumentMetadata};

pub trait TextSplit {
    /// Maximum size of chunks to return.
    fn chunk_len(&self) -> usize;

    /// Overlap in characters between chunks
    fn chunk_overlap(&self) -> usize;

    /// Splits the text into multiple components.
    fn split_text(&self, text: String) -> Vec<String>;

    /// Create documents from the chunks the text was split into.
    fn create_documents(
        &self,
        texts: Vec<String>,
        metadatas: Vec<DocumentMetadata>,
    ) -> anyhow::Result<Vec<Document>>;
}

pub trait TokenTextSplit: TextSplit {}
