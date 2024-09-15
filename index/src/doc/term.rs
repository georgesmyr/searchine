use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Builder for a `DocumentTermsCounter`
#[derive(Default)]
pub(crate) struct DocumentTermsCounterBuilder {
    doc_term_counts: HashMap<u32, u32>,
}

impl DocumentTermsCounterBuilder {
    /// Inserts the number of terms for a document with specified
    /// document ID.
    pub(crate) fn insert_doc_terms(&mut self, doc_id: u32, n_terms: u32) {
        self.doc_term_counts.insert(doc_id, n_terms);
    }

    /// Builds the `DocumentTermsCounter`
    pub(crate) fn build(self) -> DocumentTermsCounter {
        DocumentTermsCounter { inner: self.doc_term_counts }
    }
}

/// Stores the number of terms for each document, specified
/// by their document ID.
#[derive(Default, Debug, Deserialize, Serialize)]
pub(crate) struct DocumentTermsCounter {
    inner: HashMap<u32, u32>,
}

impl DocumentTermsCounter {
    /// Returns the total number of terms in a document with a
    /// specified document ID. If the document is not present
    /// it returns None.
    pub(crate) fn get_doc_terms(&self, doc_id: u32) -> u32 {
        *self.inner.get(&doc_id).unwrap_or(&0)
    }

    /// Returns the number of documents in the collection.
    pub(crate) fn n_docs(&self) -> u32 {
        self.inner.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_term_counter() {
        let mut builder = DocumentTermsCounterBuilder::default();
        builder.insert_doc_terms(0, 10);
        builder.insert_doc_terms(1, 20);
        let counter = builder.build();
        assert_eq!(counter.get_doc_terms(0), 10);
        assert_eq!(counter.get_doc_terms(1), 20);
    }
}
