use crate::postings::Posting;

/// A posting for a term in a document, containing the document ID and the
/// frequency of the term in that document.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use crate::postings::FrequencyPosting;
///
/// let mut posting = FrequencyPosting::new(1);
/// posting.add_occurrence();
/// posting.add_occurrence();
///
/// assert_eq!(postings.doc_id(), 1);
/// assert_eq!(postings.term_count(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FrequencyPosting {
    doc_id: usize,
    term_count: usize,
}

impl Posting for FrequencyPosting {
    /// Creates a new frequency posting for a term in a document with the
    /// specified document ID.
    fn new(doc_id: usize) -> Self {
        Self {
            doc_id,
            term_count: 0,
        }
    }

    /// Returns the document ID of the posting.
    fn doc_id(&self) -> usize {
        self.doc_id
    }

    /// Returns the count of the term in the document.
    fn term_count(&self) -> usize {
        self.term_count
    }
}

impl FrequencyPosting {
    /// Adds an occurrence of the term in the document, incrementing
    /// the frequency count by one.
    pub fn add_occurrence(&mut self) {
        self.term_count += 1;
    }
}
