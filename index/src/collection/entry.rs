use std::cmp::Ordering;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use documents::DocumentId;

/// A struct representing an entry in the corpus index.
/// It contains the document ID and the last time the document was modified.
///
/// The document ID is a unique identifier for each document in the corpus.
/// The last modified time is used to determine if the document has been
/// modified since the last indexing.
#[derive(Serialize, Deserialize, Clone)]
pub struct CollectionEntry {
    document_id: DocumentId,
    modified: SystemTime,
}

impl CollectionEntry {
    /// Creates a new `CollectionEntry` with specified document ID,
    /// and the last time the document was modified.
    pub fn new(document_id: DocumentId, modified: SystemTime) -> Self {
        Self {
            document_id,
            modified,
        }
    }

    /// Returns the last-modified-time of associate with the document,
    /// at the time that it was indexed.
    pub fn modified(&self) -> SystemTime {
        self.modified
    }

    /// Returns the document ID associated with the document.
    pub fn document_id(&self) -> DocumentId {
        self.document_id
    }
}

impl Ord for CollectionEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.document_id.cmp(&other.document_id)
    }
}

impl PartialOrd for CollectionEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CollectionEntry {
    fn eq(&self, other: &Self) -> bool {
        self.document_id == other.document_id
    }
}

impl Eq for CollectionEntry {}
