use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// A struct representing an entry in the corpus index.
/// It contains the document ID and the last time the document was modified.
///
/// The document ID is a unique identifier for each document in the corpus.
/// The last modified time is used to determine if the document has been
/// modified since the last indexing.
#[derive(Serialize, Deserialize, Clone)]
pub struct CorpusIndexEntry {
    pub document_id: usize,
    pub modified: SystemTime,
}

impl CorpusIndexEntry {
    /// Creates a new `CorpusIndexEntry` with specified document ID,
    /// and the last time the document was modified.
    pub fn new(document_id: usize, modified: SystemTime) -> Self {
        Self {
            document_id,
            modified,
        }
    }
}

impl Ord for CorpusIndexEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.document_id.cmp(&other.document_id)
    }
}

impl PartialOrd for CorpusIndexEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CorpusIndexEntry {
    fn eq(&self, other: &Self) -> bool {
        self.document_id == other.document_id
    }
}

impl Eq for CorpusIndexEntry {}

/// A struct representing a corpus index, which also serves as cache.
///
/// This struct is used to build an in-memory index for multiple documents.
/// Each document is assigned a unique document ID, and the last time the
/// document was indexed.
#[derive(Serialize, Deserialize)]
pub struct CorpusIndex {
    root_dir: PathBuf,
    index: HashMap<PathBuf, CorpusIndexEntry>,
    next_id: usize,
}

impl Default for CorpusIndex {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::new(),
            index: HashMap::new(),
            next_id: 0,
        }
    }
}

impl CorpusIndex {
    /// Adds a document to the index, and assigns it a unique ID.
    fn insert(&mut self, document_path: PathBuf) -> io::Result<()> {
        if !self.index.contains_key(&document_path) {
            let modified = document_path.metadata()?.modified()?;
            let entry = CorpusIndexEntry::new(self.next_id, modified);
            self.index.insert(document_path, entry);
            self.next_id += 1;
        }
        Ok(())
    }

    /// Creates a new `CorpusIndex` from an iterator of paths.
    pub fn from_paths(iter: impl IntoIterator<Item = PathBuf>) -> io::Result<Self> {
        let mut index = CorpusIndex::default();
        for path in iter {
            index.insert(path)?;
        }
        Ok(index)
    }

    /// Load the document index from a disk.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: CorpusIndex = serde_json::from_reader(reader)?;
        Ok(index)
    }

    /// Returns the document id for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    pub fn get_document_id(&self, document_path: &PathBuf) -> Option<usize> {
        Some(self.index.get(document_path)?.document_id)
    }

    /// Returns the last modified time for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    pub fn get_modified(&self, document_path: &PathBuf) -> Option<SystemTime> {
        Some(self.index.get(document_path)?.modified)
    }

    /// Write the document index to a disk.
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
}

impl IntoIterator for CorpusIndex {
    type Item = (PathBuf, CorpusIndexEntry);
    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, CorpusIndexEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}

impl<'a> IntoIterator for &'a CorpusIndex {
    type Item = (&'a PathBuf, &'a CorpusIndexEntry);
    type IntoIter = std::collections::hash_map::Iter<'a, PathBuf, CorpusIndexEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.iter()
    }
}

/// A struct representing an inverted corpus index.
///
/// This struct maps document IDs to their corresponding paths.
pub struct InvertedCorpusIndex {
    inner: HashMap<usize, PathBuf>,
}

impl InvertedCorpusIndex {
    /// Creates a new InvertedCorpusIndex from a file that stores the
    /// corpus index.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let corpus_index = CorpusIndex::from_file(path)?;
        let inner = corpus_index
            .into_iter()
            .map(|(path, entry)| (entry.document_id, path))
            .collect::<HashMap<usize, PathBuf>>();
        Ok(Self { inner })
    }

    /// Retrieves the path associated with a given document ID.
    ///
    /// # Arguments
    ///
    /// * `document_id` - The unique identifier of the document.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `PathBuf` if the document ID exists,
    /// or `None` if it does not.
    pub fn get_path(&self, document_id: usize) -> Option<&PathBuf> {
        self.inner.get(&document_id)
    }
}
