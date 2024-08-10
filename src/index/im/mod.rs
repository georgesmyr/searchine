pub mod doc;

use std::collections::HashMap;
use std::io;
use std::path::Path;

pub use doc::{InMemoryDocumentIndex, InMemoryDocumentIndexer};

/// An in-memory index for multiple documents. The index is a HashMap
/// with the document ID as the key and an in-memory document index as
/// the value.
///
/// # Examples
///
/// ```
/// use crate::index::InMemoryIndex;
/// use crate::postings::FrequencyPosting;
/// ```
#[derive(Debug)]
pub struct InMemoryIndex {
    pub index: HashMap<usize, InMemoryDocumentIndex>,
}

impl InMemoryIndex {
    /// Creates a new in-memory index.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// Creates a new in-memory index from a file.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = std::fs::File::open(path).expect("Failed to open file");
        let reader = io::BufReader::new(file);
        let index: HashMap<usize, InMemoryDocumentIndex> =
            serde_json::from_reader(reader).expect("Failed to read index from disk");
        Ok(Self { index })
    }

    /// Inserts a document index into the in-memory indexer.
    ///
    /// # Arguments
    ///
    /// * `doc_id` - The ID of the document.
    /// * `doc_index` - The in-memory document index to be inserted.
    pub fn insert(&mut self, doc_id: usize, doc_index: InMemoryDocumentIndex) {
        self.index.insert(doc_id, doc_index);
    }

    /// Returns the number of documents in the index that contain a
    /// specified term.
    pub fn n_docs_containing(&self, term: &usize) -> usize {
        self.index
            .values()
            .filter(|&index| index.contains(term))
            .count()
    }

    /// Returns the number of documents in the index.
    pub fn n_docs(&self) -> usize {
        self.index.len()
    }

    /// Writes the index to disk.
    pub fn write_to_disk(self, path: impl AsRef<Path>) {
        let file = std::fs::File::create(path).expect("Failed to create file");
        let writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.index).expect("Failed to write index to disk");
    }
}

// /// An in-memory inverted index. The inverted index is a HashMap with
// /// the term ID as the key and a postings list as the value.
// pub struct InMemoryInvertedIndex {
//     pub index: HashMap<usize, PostingsList<T>>,
// }
//
// impl<T: Posting> InMemoryInvertedIndex<T>
// where
//     T: Serialize + Deserialize<'static>,
// {
//     /// Creates a new in-memory inverted index.
//     pub fn new() -> Self {
//         Self {
//             index: HashMap::new(),
//         }
//     }
//
//     /// Inserts a document index into the in-memory inverted indexer.
//     ///
//     /// For each token in the document index, the method inserts the
//     /// token into the inverted index. If the token is already in the
//     /// index, the posting is inserted into the postings list.
//     pub fn insert(&mut self, doc_index: InMemoryDocumentIndex) {
//         for (token_id, posting) in doc_index {
//             if let Some(p_list) = self.index.get_mut(&token_id) {
//                 p_list.insert(posting);
//             } else {
//                 let mut p_list = PostingsList::new();
//                 p_list.insert(posting);
//                 self.index.insert(token_id, p_list);
//             }
//         }
//     }
//
//     /// Returns the number of documents in the index that contain a
//     /// specified term.
//     pub fn n_docs_containing(&self, token_id: &usize) -> usize {
//         self.index.get(token_id).map_or(0, |p_list| p_list.len())
//     }
//
//     /// Returns the number of documents in the index.
//     pub fn n_docs(&self) -> usize {
//         let mut docs = BTreeSet::new();
//         self.index.iter().for_each(|(_, p_list)| {
//             p_list.keys().for_each(|doc_id| {
//                 docs.insert(doc_id);
//             });
//         });
//         docs.len()
//     }
// }
