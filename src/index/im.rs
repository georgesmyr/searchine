use crate::postings::*;
use crate::scores::*;
use crate::tokenize::TokenIds;
use std::collections::{HashMap, HashSet};

pub struct InMemoryIndex<T> {
    pub index: HashMap<usize, InMemoryDocumentIndex<T>>,
}

impl<T: Posting> InMemoryIndex<T> {
    /// Returns the number of documents in the index that contain a
    /// specified term.
    pub fn n_docs_containing(&self, term: &usize) -> usize {
        self.index
            .values()
            .filter(|&index| index.contains(term))
            .count()
    }

    /// Returns the number of documents in the index.
    pub fn n_docs(&self) -> usize {
        self.index.len()
    }

    /// Calculates the inverse document frequency of a term.
    pub fn calc_idf(&self, term: &usize) -> f64 {
        idf(self.n_docs_containing(term), self.n_docs())
    }

    /// Calculate the TF-IDF score of a term in a document.
    pub fn score_tf_idf(&self, doc_id: usize, term: &usize) -> f64 {
        if let Some(doc_index) = self.index.get(&doc_id) {
            let tf = doc_index.term_frequency(term);
            let idf = self.calc_idf(term);
            tf_idf(tf, idf)
        } else {
            0.0
        }
    }
}


/// A struct representing an in-memory indexer.
///
/// This struct is used to build an in-memory index for multiple documents.
/// The index is stored in memory and can be finalized to an `InMemoryIndex`.
///
/// # Type Parameters
///
/// - `T`: The type of the postings list to be stored in the index.
pub struct InMemoryIndexer<T> {
    index: HashMap<usize, InMemoryDocumentIndex<T>>,
}

impl<T> InMemoryIndexer<T> {
    /// Creates a new in-memory indexer.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// Inserts a document index into the in-memory indexer.
    ///
    /// # Arguments
    ///
    /// * `doc_id` - The ID of the document.
    /// * `doc_index` - The in-memory document index to be inserted.
    pub fn insert(&mut self, doc_id: usize, doc_index: InMemoryDocumentIndex<T>) {
        self.index.insert(doc_id, doc_index);
    }

    /// Finalizes the indexing process, consuming the indexer and returning
    /// an `InMemoryIndex`.
    pub fn finalize(self) -> InMemoryIndex<T> {
        InMemoryIndex {
            index: self.index,
        }
    }
}


/// A struct representing an in-memory document index.
///
/// This struct is used to store the postings of a single document.
///
/// # Type Parameters
///
/// - `T`: The type of the postings list to be stored in the index.
///
/// # Examples
///
/// ```
/// use crate::index::postings::{FrequencyPosting, Posting};
/// use crate::index::spimi::InMemoryDocumentIndex;
///
/// let index = InMemoryDocumentIndex::new(0, HashMap::from([
///     ("hello".to_string(), FrequencyPosting::new(0, 2)),
///     ("world".to_string(), FrequencyPosting::new(0, 1)),
/// ]));
///
/// assert_eq!(index.doc_id(), 0);
/// assert_eq!(index.n_terms(), 2);
/// assert_eq!(index.get("hello").unwrap().term_frequency(), 2);
/// assert_eq!(index.get("world").unwrap().term_frequency(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct InMemoryDocumentIndex<T> {
    doc_id: usize,
    index: HashMap<usize, T>,
}

impl<T: Posting> InMemoryDocumentIndex<T> {
    /// Returns the document ID of the index.
    pub fn doc_id(&self) -> usize {
        self.doc_id
    }

    /// Returns True if the index contains a term. Otherwise, False.
    pub fn contains(&self, term: &usize) -> bool {
        self.index.contains_key(term)
    }

    /// Returns a shared reference to the posting of a term.
    pub fn get(&self, term: &usize) -> Option<&T> {
        self.index.get(term)
    }

    /// Returns the length of the index, i.e. the number of terms.
    pub fn n_terms(&self) -> usize {
        self.index.len()
    }

    /// Returns the number of occurrences of a term in the document.
    pub fn term_count(&self, term: &usize) -> usize {
        if let Some(posting) = self.get(term) {
            posting.term_count()
        } else {
            0
        }
    }

    /// Returns the term frequency of a term in the document.
    pub fn term_frequency(&self, term: &usize) -> f64 {
        tf(self.term_count(term), self.index.len())
    }
}

impl<T> IntoIterator for InMemoryDocumentIndex<T> {
    type Item = (usize, T);
    type IntoIter = std::collections::hash_map::IntoIter<usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}

/// A struct representing an in-memory document indexer.
///
/// This struct is used to index tokens for a single document, identified by `doc_id`.
/// The index is stored in memory and can be finalized to an `InMemoryDocumentIndex`.
///
/// # Type Parameters
///
/// - `T`: The type of the postings list to be stored in the index.
///
/// # Examples
///
/// ```
/// use crate::index::postings::{FrequencyPosting, Posting};
/// use crate::index::spimi::InMemoryDocumentIndexer;
///
/// let mut indexer = InMemoryDocumentIndexer::<FrequencyPosting>::new(0);
/// indexer.index_tokens(vec![
///    "hello".to_string(),
///    "world".to_string(),
///    "hello".to_string(),
/// ]);
/// let index = indexer.finalize();
/// assert_eq!(index, InMemoryDocumentIndex::new(0, HashMap::from([
///   ("hello".to_string(), FrequencyPosting::new(0, 2)),
///   ("world".to_string(), FrequencyPosting::new(0, 1)),
/// ])));
/// ```
pub struct InMemoryDocumentIndexer<T> {
    doc_id: usize,
    index: HashMap<usize, T>,
}

impl<T> InMemoryDocumentIndexer<T> {
    /// Creates a new in-memory indexer for a single document with
    /// specified document ID.
    pub fn new(doc_id: usize) -> Self {
        Self {
            doc_id,
            index: HashMap::new(),
        }
    }

    /// Finalizes the indexing returning the in-memory index, consuming
    /// the indexer.
    pub fn finalize(self) -> InMemoryDocumentIndex<T> {
        InMemoryDocumentIndex {
            doc_id: self.doc_id,
            index: self.index,
        }
    }
}

impl InMemoryDocumentIndexer<FrequencyPosting> {
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
        if let Some(posting) = self.index.get_mut(&token) {
            posting.add_occurrence();
        } else {
            let mut posting = FrequencyPosting::new(self.doc_id);
            posting.add_occurrence();
            self.index.insert(token, posting);
        }
    }
}

impl InMemoryDocumentIndexer<PositionsPosting> {
    /// Indexes a list of tokens.
    pub fn index_tokens(&mut self, tokens: Vec<usize>) {
        for (position, token) in tokens.into_iter().enumerate() {
            self.add_token(token, position);
        }
    }

    /// Adds a token to the index.
    ///
    /// If the token is already in the index, the position is added to the posting.
    /// Otherwise, a new posting is created.
    fn add_token(&mut self, token: usize, position: usize) {
        if let Some(posting) = self.index.get_mut(&token) {
            posting.insert_position(position);
        } else {
            let mut posting = PositionsPosting::new(self.doc_id);
            posting.insert_position(position);
            self.index.insert(token, posting);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashSet;
//
//     #[test]
//     fn test_in_memory_index() {
//         let mut indexer = InMemoryDocumentIndexer::<FrequencyPosting>::new(0);
//         indexer.index_tokens(vec![
//             "hello".to_string(),
//             "world".to_string(),
//             "hello".to_string(),
//             "world".to_string(),
//             "hello".to_string(),
//             "world".to_string(),
//         ]);
//         let index = indexer.finalize();
//         assert_eq!(index.n_terms(), 2);
//         assert_eq!(index.get("hello").unwrap().term_count(), 3);
//         assert_eq!(index.get("world").unwrap().term_count(), 3);
//     }
//
//     #[test]
//     fn test_in_memory_index_positions() {
//         let mut indexer = InMemoryDocumentIndexer::<PositionsPosting>::new(0);
//         indexer.index_tokens(vec![
//             "hello".to_string(),
//             "world".to_string(),
//             "hello".to_string(),
//             "world".to_string(),
//             "hello".to_string(),
//             "world".to_string(),
//         ]);
//         let index = indexer.finalize();
//         assert_eq!(index.n_terms(), 2);
//         assert_eq!(
//             index.get("hello").unwrap().term_positions(),
//             &HashSet::from([0, 2, 4])
//         );
//         assert_eq!(
//             index.get("world").unwrap().term_positions(),
//             &HashSet::from([1, 3, 5])
//         );
//     }
// }
