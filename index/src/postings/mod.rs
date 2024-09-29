use documents::DocumentId;
pub(crate) use freq::{FrequencyPosting, FrequencyPostingsList};

pub(crate) mod freq;
mod pos;

pub(crate) trait Posting {
    /// Returns the document id of the posting.
    fn doc_id(&self) -> DocumentId;
    /// Returns the frequency of the term in the document.
    fn frequency(&self) -> u32;
}

pub(crate) trait PostingsList<P>
where
    P: Posting,
{
    /// Adds a posting to the postings list.
    fn add(&mut self, posting: P);
    /// Removes the posting of the specified document ID.
    fn remove(&mut self, doc_id: DocumentId);
    /// Returns the posting of the specified document ID.
    fn get(&self, doc_id: DocumentId) -> Option<&P>;
    /// Returns the number of postings in the list.
    fn len(&self) -> usize;
    /// Returns the document IDs
    fn doc_ids(&self) -> Vec<DocumentId>;
}
