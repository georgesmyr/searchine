use crate::postings::Posting;
use std::collections::HashMap;

/// A list of postings for a specific term. Each posting in the list
/// corresponds to a document in which the term appears.
///
/// The postings are stored in a HashMap with the document ID as the key.
///
/// # Examples
///
/// ```
/// use crate::index::postings::{FrequencyPosting, PostingsList};
///
/// let mut postings = PostingsList::new();
///
/// let mut posting_1 = FrequencyPosting::new(1);
/// posting_1.add_occurrence();
/// posting_1.add_occurrence();
/// postings.insert_posting(posting_1);
/// postings.get_mut(1).unwrap().add_occurrence();
///
/// let mut posting_2 = FrequencyPosting::new(2);
/// posting_2.add_occurrence();
/// postings.insert_posting(posting_2);
///
/// assert_eq!(postings.get(1).unwrap().term_frequency(), 3);
/// assert_eq!(postings.get(2).unwrap().term_frequency(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct PostingsList<T> {
    /// HashMap of postings `T` with the document ID as the key.
    postings: HashMap<usize, T>,
}

impl<T: Posting> PostingsList<T> {
    /// Creates a new, empty list of postings.
    pub fn new() -> Self {
        Self {
            postings: HashMap::new(),
        }
    }

    /// Inserts a posting into the postings list.
    pub fn insert(&mut self, posting: T) {
        self.postings.insert(posting.doc_id(), posting);
    }

    /// Returns a mutable reference to the posting of the term, for
    /// specified document ID.
    pub fn get_mut(&mut self, doc_id: usize) -> Option<&mut T> {
        self.postings.get_mut(&doc_id)
    }

    /// Returns a shared reference to the posting of the term, for
    /// specified document ID.
    pub fn get(&self, doc_id: usize) -> Option<&T> {
        self.postings.get(&doc_id)
    }
}

impl<'a, T> IntoIterator for &'a PostingsList<T> {
    type Item = (&'a usize, &'a T);
    type IntoIter = std::collections::hash_map::Iter<'a, usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.postings.iter()
    }
}
