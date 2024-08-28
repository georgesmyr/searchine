pub(crate) mod freq;
mod pos;

pub(crate) use freq::{FrequencyPosting, FrequencyPostingsList};

pub(crate) trait Posting {
    /// Returns the document id of the posting.
    fn doc_id(&self) -> usize;
    /// Returns the frequency of the term in the document.
    fn frequency(&self) -> usize;
}

pub(crate) trait PostingsList<P>
where
    P: Posting,
{
    /// Adds a posting to the postings list.
    fn add(&mut self, posting: P);
    /// Removes the posting of the specified document ID.
    fn remove(&mut self, doc_id: usize);
    /// Returns the posting of the specified document ID.
    fn get(&self, doc_id: usize) -> Option<&P>;
    /// Returns the number of postings in the list.
    fn len(&self) -> usize;
    /// Returns the document IDs
    fn doc_ids(&self) -> Vec<usize>;
}
