pub use freq::*;
pub use lst::*;
pub use pos::*;

pub mod freq;
pub mod lst;
pub mod pos;

pub trait Posting {
    /// Creates a new posting with the given document ID.
    fn new(doc_id: usize) -> Self;
    /// Returns the document ID of the posting.
    fn doc_id(&self) -> usize;
    /// Returns the count of the term in the document.
    fn term_count(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_frequency_posting() {
        let mut posting = FrequencyPosting::new(1);
        posting.add_occurrence();
        posting.add_occurrence();
        assert_eq!(posting.doc_id(), 1);
        assert_eq!(posting.term_count(), 2);
    }

    #[test]
    fn test_positions_posting() {
        let mut posting = PositionsPosting::new(1);
        posting.insert_position(2);
        posting.insert_position(3);
        assert_eq!(posting.doc_id(), 1);
        assert_eq!(posting.term_positions(), &HashSet::from([2, 3]));
        assert_eq!(posting.term_count(), 2);
    }

    #[test]
    fn test_frequency_postings_list() {
        let mut postings = PostingsList::<FrequencyPosting>::new();

        let mut posting_1 = FrequencyPosting::new(1);
        posting_1.add_occurrence();
        posting_1.add_occurrence();
        postings.insert(posting_1);
        postings.get_mut(1).unwrap().add_occurrence();

        let mut posting_2 = FrequencyPosting::new(2);
        posting_2.add_occurrence();
        postings.insert(posting_2);

        assert_eq!(postings.get(1).unwrap().term_count(), 3);
        assert_eq!(postings.get(2).unwrap().term_count(), 1);
    }

    #[test]
    fn test_positions_postings_list() {
        let mut postings = PostingsList::<PositionsPosting>::new();

        let mut posting_1 = PositionsPosting::new(1);
        posting_1.insert_position(2);
        posting_1.insert_position(3);
        postings.insert(posting_1);

        let mut posting_2 = PositionsPosting::new(2);
        posting_2.insert_position(4);
        postings.insert(posting_2);

        assert_eq!(
            postings.get(1).unwrap().term_positions(),
            &HashSet::from([2, 3])
        );
        assert_eq!(
            postings.get(2).unwrap().term_positions(),
            &HashSet::from([4])
        );
    }
}
