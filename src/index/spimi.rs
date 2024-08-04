use crate::postings::*;
use crate::types::Tokens;
use std::collections::{HashMap, HashSet};

pub type InMemoryIndex<T> = HashMap<String, InMemoryDocumentIndex<T>>;

pub struct InMemoryInvertedIndex<T> {
    pub index: HashMap<String, PostingsList<T>>,
}

impl<T: Posting> InMemoryInvertedIndex<T> {
    pub fn get_score(&self, doc_id: usize, term: &str) -> f64 {
        // Calculate the number of occurrences in the document.
        let tf = self.get_frequency(doc_id, term) as f64;
        // Calculate the number of occurrences in all documents.
        let itf = self.get_freq_all(term) as f64;
        tf / itf
    }

    pub fn get_frequency(&self, doc_id: usize, term: &str) -> usize {
        self.index.get(term).unwrap().get(doc_id).unwrap().term_count()
    }

    pub fn get_freq_all(&self, term: &str) -> usize {
        let posting_list = self.index.get(term).unwrap();
        (&posting_list)
            .into_iter()
            .fold(0, |acc, (_, posting)| acc + posting.term_count())
    }
}


pub struct InMemoryInvertedIndexer<T> {
    index: HashMap<String, PostingsList<T>>,
}

impl<T: Posting> InMemoryInvertedIndexer<T> {
    /// Creates a new in-memory indexer for a corpus of documents.
    pub fn new() -> Self { Self { index: HashMap::new() } }

    /// Adds an in-memory document index to the corpus index.
    pub fn add_document_index(&mut self, index: InMemoryDocumentIndex<T>) {
        for (term, posting) in index {
            if let Some(postings_list) = self.index.get_mut(&term) {
                postings_list.insert(posting);
            } else {
                let mut posting_list = PostingsList::new();
                posting_list.insert(posting);
                self.index.insert(term, posting_list);
            }
        }
    }

    /// Finalizes the indexing returning the in-memory index, consuming
    /// the indexer.
    pub fn finalize(self) -> InMemoryInvertedIndex<T> {
        InMemoryInvertedIndex { index: self.index }
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
/// assert_eq!(index.len(), 2);
/// assert_eq!(index.get("hello").unwrap().term_frequency(), 2);
/// assert_eq!(index.get("world").unwrap().term_frequency(), 1);
/// ```
pub struct InMemoryDocumentIndex<T> {
    doc_id: usize,
    index: HashMap<String, T>,
}

impl<T> InMemoryDocumentIndex<T> {
    /// Returns the length of the index, i.e. the number of terms.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Returns the document ID of the index.
    pub fn doc_id(&self) -> usize {
        self.doc_id
    }

    /// Returns a shared reference to the posting of a term.
    pub fn get(&self, term: &str) -> Option<&T> {
        self.index.get(term)
    }
}

impl<T> IntoIterator for InMemoryDocumentIndex<T> {
    type Item = (String, T);
    type IntoIter = std::collections::hash_map::IntoIter<String, T>;

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
    index: HashMap<String, T>,
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
    pub fn index_tokens(&mut self, tokens: Tokens) {
        for token in tokens {
            self.add_token(token);
        }
    }

    /// Adds a token to the index.
    ///
    /// If the token is already in the index, the frequency count is incremented.
    /// Otherwise, a new posting is created.
    fn add_token(&mut self, token: String) {
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
    pub fn index_tokens(&mut self, tokens: Tokens) {
        for (position, token) in tokens.into_iter().enumerate() {
            self.add_token(token, position);
        }
    }

    /// Adds a token to the index.
    ///
    /// If the token is already in the index, the position is added to the posting.
    /// Otherwise, a new posting is created.
    fn add_token(&mut self, token: String, position: usize) {
        if let Some(posting) = self.index.get_mut(&token) {
            posting.insert_position(position);
        } else {
            let mut posting = PositionsPosting::new(self.doc_id);
            posting.insert_position(position);
            self.index.insert(token, posting);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_in_memory_index() {
        let mut indexer = InMemoryDocumentIndexer::<FrequencyPosting>::new(0);
        indexer.index_tokens(vec![
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ]);
        let index = indexer.finalize();
        assert_eq!(index.len(), 2);
        assert_eq!(index.get("hello").unwrap().term_count(), 3);
        assert_eq!(index.get("world").unwrap().term_count(), 3);
    }

    #[test]
    fn test_in_memory_index_positions() {
        let mut indexer = InMemoryDocumentIndexer::<PositionsPosting>::new(0);
        indexer.index_tokens(vec![
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ]);
        let index = indexer.finalize();
        assert_eq!(index.len(), 2);
        assert_eq!(
            index.get("hello").unwrap().term_positions(),
            &HashSet::from([0, 2, 4])
        );
        assert_eq!(
            index.get("world").unwrap().term_positions(),
            &HashSet::from([1, 3, 5])
        );
    }
}
