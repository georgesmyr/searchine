use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A struct representing an in-memory document indexer.
///
/// This struct is used to index tokens for a single document, identified by `doc_id`.
/// The index is stored in memory and can be finalized to an `InMemoryDocumentIndex`.
pub struct DocumentFrequencyIndexer {
    /// Count of the total number of terms in the document.
    n_terms: usize,
    id: usize,
    index: HashMap<String, usize>,
}

impl DocumentFrequencyIndexer {
    /// Creates new indexer for document with specified document ID.
    pub fn new(id: usize) -> Self {
        Self {
            n_terms: 0,
            id,
            index: HashMap::new(),
        }
    }

    /// Adds a token to the index.
    ///
    /// If the token is already in the index, the frequency count is incremented.
    /// Otherwise, a new posting is created.
    fn add_token(&mut self, token: String) {
        self.n_terms += 1;
        if let Some(posting) = self.index.get_mut(&token) {
            *posting += 1
        } else {
            self.index.insert(token, 1);
        }
    }

    /// Indexes a list of tokens.
    pub fn index_tokens(&mut self, tokens: impl IntoIterator<Item=String>) {
        for token in tokens {
            self.add_token(token);
        }
    }

    /// Builds the index.
    pub fn build(self) -> DocumentFrequencyIndex {
        DocumentFrequencyIndex {
            n_terms: self.n_terms,
            id: self.id,
            index: self.index,
        }
    }
}

/// A struct representing an in-memory document index.
///
/// This struct is used to store the postings of a single document, i.e.
/// for the same document, each term in the document  is associated with
/// a posting, which is the frequency of the word in the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentFrequencyIndex {
    id: usize,
    n_terms: usize,
    index: HashMap<String, usize>,
}

impl DocumentFrequencyIndex {
    /// Returns the ID of the document that the document index is
    /// referring to.
    pub fn doc_id(&self) -> usize {
        self.id
    }

    /// Returns the total number of terms in the document.
    pub fn n_terms(&self) -> usize {
        self.n_terms
    }

    /// Returns the number of occurrences of a term in the document.
    pub fn term_count(&self, term: &str) -> usize {
        *self.index.get(term).unwrap_or(&0)
    }
}

impl IntoIterator for DocumentFrequencyIndex {
    type Item = (String, usize);
    type IntoIter = std::collections::hash_map::IntoIter<String, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}
