use crate::postings::{Posting, PostingsList};

use std::hash::{Hash, Hasher};
use std::collections::HashSet;

use serde::{Serialize, Deserialize};


/// Structure that represents a frequency-posting for a term.
/// It contains the document ID and the frequency of the term in the document.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PositionPosting {
    doc_id: usize,
    positions: HashSet<usize>,
}

impl PositionPosting {
    /// Creates a new frequency-posting, by specifying the document ID
    /// and the frequency.
    pub fn new(doc_id: usize) -> Self {
        Self { doc_id, positions: HashSet::new() }
    }

    /// Adds positions in the `PositionPosting`.
    fn add_position(&mut self, pos: usize) {
        self.positions.insert(pos);
    }
}

impl Posting for PositionPosting {
    /// Returns the document ID of the frequency-posting.
    fn doc_id(&self) -> usize {
        self.doc_id
    }

    /// Returns the frequency of the term in the document.
    fn frequency(&self) -> usize {
        self.positions.len()
    }
}

impl PartialEq for PositionPosting {
    fn eq(&self, other: &Self) -> bool {
        self.doc_id == other.doc_id
    }
}

impl Eq for PositionPosting {}

impl Hash for PositionPosting {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.doc_id.hash(state);
    }
}

/// Structure that represents a list of frequency-postings.
#[derive(Default)]
pub(crate) struct PositionsPostingsList {
    inner: HashSet<PositionPosting>,
}

impl PositionsPostingsList {
    /// Creates a new empty frequency-postings list.
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl PostingsList<PositionPosting> for PositionsPostingsList {
    fn add(&mut self, posting: PositionPosting) {
        self.inner.insert(posting);
    }
    fn remove(&mut self, doc_id: usize) {
        self.inner.retain(|posting| posting.doc_id() != doc_id);
    }
    fn get(&self, doc_id: usize) -> Option<&PositionPosting> {
        self.inner.iter().find(|posting| posting.doc_id() == doc_id)
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
    fn doc_ids(&self) -> Vec<usize> {
        self.inner.iter()
            .map(|posting| posting.doc_id())
            .collect::<Vec<_>>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_posting() {
        let mut posting = PositionPosting::new(1);
        posting.add_position(1);
        posting.add_position(10);
        assert_eq!(posting.doc_id(), 1);
        assert_eq!(posting.frequency(), 2);
    }

    #[test]
    fn test_positions_postings_list() {
        let mut postings_list = PositionsPostingsList::new();
        let mut pos1 = PositionPosting::new(1);
        pos1.add_position(2);
        pos1.add_position(10);
        let mut pos2 = PositionPosting::new(2);
        pos2.add_position(4);
        pos2.add_position(6);
        pos2.add_position(8);
        let mut pos3 = PositionPosting::new(3);
        pos3.add_position(5);
        postings_list.add(pos1);
        postings_list.add(pos2);
        postings_list.add(pos3);

        assert_eq!(postings_list.len(), 3);

        let posting = postings_list.get(2).unwrap();
        assert_eq!(posting.doc_id(), 2);
        assert_eq!(posting.frequency(), 3);

        postings_list.remove(2);
        assert_eq!(postings_list.len(), 2);
        assert!(postings_list.get(2).is_none());
    }
}
