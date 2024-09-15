use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::Context;
use serde::{Deserialize, Serialize};

/// A struct representing an entry in the corpus index.
/// It contains the document ID and the last time the document was modified.
///
/// The document ID is a unique identifier for each document in the corpus.
/// The last modified time is used to determine if the document has been
/// modified since the last indexing.
#[derive(Serialize, Deserialize, Clone)]
pub struct CorpusIndexEntry {
    document_id: u32,
    modified: SystemTime,
}

impl CorpusIndexEntry {
    /// Creates a new `CorpusIndexEntry` with specified document ID,
    /// and the last time the document was modified.
    pub fn new(document_id: u32, modified: SystemTime) -> Self {
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
    pub fn document_id(&self) -> u32 {
        self.document_id
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
    next_id: u32,
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
    pub fn insert(&mut self, document_path: PathBuf) -> io::Result<()> {
        if !self.index.contains_key(&document_path) {
            let modified = document_path.metadata()?.modified()?;
            let entry = CorpusIndexEntry::new(self.next_id, modified);
            self.index.insert(document_path, entry);
            self.next_id += 1;
        }
        Ok(())
    }

    /// Creates a new `CorpusIndex` from an iterator of paths.
    pub fn from_paths(iter: impl IntoIterator<Item=PathBuf>) -> io::Result<Self> {
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

    /// Returns true if the index contains a document with the specified path.
    /// Otherwise, it returns false.
    pub fn contains_path(&self, document_path: &PathBuf) -> bool {
        self.index.contains_key(document_path)
    }

    /// Returns the document id for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    pub fn get_document_id(&self, document_path: &PathBuf) -> Option<u32> {
        Some(self.index.get(document_path)?.document_id)
    }

    /// Returns the last modified time for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    ///
    /// # Returns
    ///
    /// An `Option` containing the last modified time if the document exists,
    /// or `None` if it does not.
    pub fn get_last_modified(&self, document_path: &PathBuf) -> Option<SystemTime> {
        Some(self.index.get(document_path)?.modified)
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

    /// Removes an index entry with the specified document path.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed `CorpusIndexEntry` if it exists,
    /// or `None` if it does not.
    pub fn remove(&mut self, document_path: &PathBuf) -> Option<CorpusIndexEntry> {
        self.index.remove(document_path)
    }

    /// Write the document index to a disk.
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
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

pub struct InvertedCollection {
    root_dir: PathBuf,
    inner: HashMap<u32, PathBuf>,
}

impl InvertedCollection {
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let collection = CorpusIndex::from_file(path)
            .context("Failed to load collection from file.")?;
        let inv = collection.index.iter()
            .map(|(path, entry)| { (entry.document_id, path.clone()) })
            .collect::<HashMap<u32, PathBuf>>();
        Ok(InvertedCollection { root_dir: collection.root_dir, inner: inv })
    }

    pub fn get_path(&self, doc_id: u32) -> Option<&PathBuf> {
        self.inner.get(&doc_id)
    }
}