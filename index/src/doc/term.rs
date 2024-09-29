use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use documents::DocumentId;

/// Stores the number of terms for each document, specified
/// by their document ID.
#[derive(Default, Debug, Deserialize, Serialize)]
pub(crate) struct DocumentTermsCounter {
    inner: HashMap<u32, u32>,
    total: u32,
}

impl DocumentTermsCounter {
    /// Inserts the number of terms for a document with specified
    /// document ID.
    pub(crate) fn insert_doc_terms(&mut self, doc_id: DocumentId, n_terms: u32) {
        self.inner.insert(doc_id, n_terms);
        self.total += n_terms;
    }

    /// Returns the total number of terms in a document with a
    /// specified document ID. If the document is not present
    /// it returns None.
    pub(crate) fn n_terms(&self, doc_id: DocumentId) -> u32 {
        *self.inner.get(&doc_id).unwrap_or(&0)
    }

    /// Returns the number of documents in the collection.
    pub(crate) fn n_docs(&self) -> u32 {
        self.inner.len() as u32
    }

    /// Returns the total number of terms in all the documents.
    pub(crate) fn n_terms_total(&self) -> u32 {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_term_counter() {
        let mut counter = DocumentTermsCounter::default();
        counter.insert_doc_terms(0, 10);
        counter.insert_doc_terms(1, 20);
        assert_eq!(counter.n_terms(0), 10);
        assert_eq!(counter.n_terms(1), 20);
    }
}
