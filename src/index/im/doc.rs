use crate::postings::{FrequencyPosting, PositionsPosting, Posting};
use std::collections::HashMap;

/// A struct representing an in-memory document index.
///
/// This struct is used to store the postings of a single document, i.e.
/// for the same document, each term in the document  is associated with
/// a posting.
///
/// # Type Parameters
///
/// - `T`: The type of the postings list to be stored in the index.
///
/// # Examples
///
/// ```
/// use crate::index::postings::{FrequencyPosting, Posting};
/// use crate::index::im::InMemoryDocumentIndex;
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
