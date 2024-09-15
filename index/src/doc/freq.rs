use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A struct representing an in-memory document frequency index.
///
/// This struct is used to store the postings of a single document.
/// Each term in the document is associated with a frequency, which
/// is the number of times the term appears in the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentFrequencyIndex {
    id: u32,
    n_terms: u32,
    index: HashMap<u32, u32>,
}

impl DocumentFrequencyIndex {
    pub fn new(id: u32) -> Self {
        Self {
            n_terms: 0,
            id,
            index: HashMap::new(),
        }
    }

    /// Adds a token to the index.
    ///
    /// If the token is already in the index, the frequency count is
    /// incremented by one. Otherwise, a new posting is created.
    fn add_token(&mut self, token: u32) {
        self.n_terms += 1;
        if let Some(posting) = self.index.get_mut(&token) {
            *posting += 1
        } else {
            self.index.insert(token, 1);
        }
    }

    /// Indexes an iterator of tokens.
    pub fn index_tokens(&mut self, tokens: impl IntoIterator<Item=u32>) {
        for token in tokens {
            self.add_token(token);
        }
    }

    /// Returns the ID of the document that the document index is
    /// referring to.
    pub fn doc_id(&self) -> u32 {
        self.id
    }

    /// Returns the total number of terms in the document.
    pub fn n_terms(&self) -> u32 {
        self.n_terms
    }

    /// Returns the number of occurrences of a term in the document.
    pub fn term_count(&self, term_id: u32) -> u32 {
        *self.index.get(&term_id).unwrap_or(&0)
    }
}

impl IntoIterator for DocumentFrequencyIndex {
    type Item = (u32, u32);
    type IntoIter = std::collections::hash_map::IntoIter<u32, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}
