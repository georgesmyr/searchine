use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use documents::DocumentId;

use crate::postings::{Posting, PostingsList};

/// Structure that represents a frequency-posting for a term.
/// It contains the document ID and the frequency of the term in the document.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FrequencyPosting {
    doc_id: DocumentId,
    frequency: u32,
}

impl FrequencyPosting {
    /// Creates a new frequency-posting, by specifying the document ID
    /// and the frequency.
    pub fn new(doc_id: DocumentId, frequency: u32) -> Self {
        Self { doc_id, frequency }
    }
}

impl Posting for FrequencyPosting {
    /// Returns the document ID of the frequency-posting.
    fn doc_id(&self) -> DocumentId {
        self.doc_id
    }

    /// Returns the frequency of the term in the document.
    fn frequency(&self) -> u32 {
        self.frequency
    }
}

impl PartialEq for FrequencyPosting {
    fn eq(&self, other: &Self) -> bool {
        self.doc_id == other.doc_id
    }
}

impl Eq for FrequencyPosting {}

impl Hash for FrequencyPosting {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.doc_id.hash(state);
    }
}

/// Structure that represents a list of frequency-postings.
#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct FrequencyPostingsList {
    inner: HashSet<FrequencyPosting>,
}

impl FrequencyPostingsList {
    /// Creates a new empty frequency-postings list.
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl PostingsList<FrequencyPosting> for FrequencyPostingsList {
    fn add(&mut self, posting: FrequencyPosting) {
        self.inner.insert(posting);
    }
    fn remove(&mut self, doc_id: DocumentId) {
        self.inner.retain(|posting| posting.doc_id() != doc_id);
    }
    fn get(&self, doc_id: DocumentId) -> Option<&FrequencyPosting> {
        self.inner.iter().find(|posting| posting.doc_id() == doc_id)
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
    fn doc_ids(&self) -> Vec<DocumentId> {
        self.inner
            .iter()
            .map(|posting| posting.doc_id())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_posting() {
        let posting = FrequencyPosting::new(1, 5);
        assert_eq!(posting.doc_id(), 1);
        assert_eq!(posting.frequency(), 5);
    }

    #[test]
    fn test_frequency_postings_list() {
        let mut postings_list = FrequencyPostingsList::new();
        postings_list.add(FrequencyPosting::new(1, 5));
        postings_list.add(FrequencyPosting::new(2, 3));
        postings_list.add(FrequencyPosting::new(3, 7));

        assert_eq!(postings_list.len(), 3);

        let posting = postings_list.get(2).unwrap();
        assert_eq!(posting.doc_id(), 2);
        assert_eq!(posting.frequency(), 3);

        postings_list.remove(2);
        assert_eq!(postings_list.len(), 2);
        assert!(postings_list.get(2).is_none());
    }
}
