use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A struct representing an in-memory document index.
///
/// This struct is used to store the postings of a single document, i.e.
/// for the same document, each term in the document  is associated with
/// a posting, which is the frequency of the word in the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InMemoryDocumentIndex {
    /// Count of the total number of terms in the document.
    count: usize,
    index: HashMap<usize, usize>,
}

impl InMemoryDocumentIndex {
    /// Returns True if the index contains a term. Otherwise, False.
    pub fn contains(&self, term: &usize) -> bool {
        self.index.contains_key(term)
    }

    /// Returns a shared reference to the posting of a term.
    pub fn get(&self, term: &usize) -> Option<&usize> {
        self.index.get(term)
    }

    /// Returns the total number of terms in the document.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the number of occurrences of a term in the document.
    pub fn term_count(&self, term: &usize) -> usize {
        if let Some(posting) = self.get(term) {
            *posting
        } else {
            0
        }
    }
}

impl IntoIterator for InMemoryDocumentIndex {
    type Item = (usize, usize);
    type IntoIter = std::collections::hash_map::IntoIter<usize, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}

/// A struct representing an in-memory document indexer.
///
/// This struct is used to index tokens for a single document, identified by `doc_id`.
/// The index is stored in memory and can be finalized to an `InMemoryDocumentIndex`.
pub struct InMemoryDocumentIndexer {
    /// Count of the total number of terms in the document.
    count: usize,
    index: HashMap<usize, usize>,
}

impl InMemoryDocumentIndexer {
    /// Creates a new in-memory indexer for a single document with
    /// specified document ID.
    pub fn new() -> Self {
        Self {
            count: 0,
            index: HashMap::new(),
        }
    }

    /// Finalizes the indexing returning the in-memory index, consuming
    /// the indexer.
    pub fn finalize(self) -> InMemoryDocumentIndex {
        InMemoryDocumentIndex {
            count: self.count,
            index: self.index,
        }
    }
}

impl InMemoryDocumentIndexer {
    /// Indexes a list of tokens.
    pub fn index_tokens(&mut self, tokens: Vec<usize>) {
        for token in tokens {
            self.add_token(token);
        }
    }

    /// Adds a token to the index.
    ///
    /// If the token is already in the index, the frequency count is incremented.
    /// Otherwise, a new posting is created.
    fn add_token(&mut self, token: usize) {
        self.count += 1;
        if let Some(posting) = self.index.get_mut(&token) {
            *posting += 1
        } else {
            self.index.insert(token, 1);
        }
    }
}
