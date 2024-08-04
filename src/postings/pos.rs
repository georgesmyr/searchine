use crate::postings::Posting;
use std::collections::HashSet;

/// A posting for a term in a document, containing the document ID and the
/// positions of the term in the document.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use crate::postings::PositionsPosting;
///
/// let mut posting = PositionsPosting::new(1);
/// posting.add_term_position(2);
/// posting.add_term_position(3);
///
/// assert_eq!(postings.doc_id(), 1);
/// assert_eq!(postings.term_positions(), &HashMap::from([2, 3]));
/// assert_eq!(postings.term_count(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PositionsPosting {
    doc_id: usize,
    term_pos: HashSet<usize>,
}

impl Posting for PositionsPosting {
    /// Creates a new positions posting for a term in a document with the
    /// specified document ID.
    fn new(doc_id: usize) -> Self {
        Self {
            doc_id,
            term_pos: HashSet::new(),
        }
    }

    /// Returns the document ID of the posting.
    fn doc_id(&self) -> usize {
        self.doc_id
    }

    /// Returns the count of the term in the document.
    fn term_count(&self) -> usize {
        self.term_pos.len()
    }
}

impl PositionsPosting {
    /// Inserts a term position into the posting.
    pub fn insert_position(&mut self, term_pos: usize) {
        self.term_pos.insert(term_pos);
    }

    /// Returns the term positions in the document.
    pub fn term_positions(&self) -> &HashSet<usize> {
        &self.term_pos
    }
}
