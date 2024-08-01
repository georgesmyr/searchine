use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;

/// A collection of documents grouped together for processing.
///
/// Grouping documents into blocks helps manage memory usage by avoiding the
/// need to process all documents in-memory at once. This approach also reduces
/// the number of disk reads by processing documents in larger chunks.
pub struct Block {
    entries: Vec<PathBuf>,
    size: u64,
}

impl Block {
    /// Creates new empty block of documents.
    pub fn new() -> Self {
        Self { entries: Vec::new(), size: 0 }
    }

    /// Returns the total size of the block in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Adds a document to the block, by specifying its path and size.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crate::fs::Block;
    ///
    /// let mut block = Block::new();
    /// block.add_entry(PathBuf::from("path/to/document.txt"), 1024);
    /// block.add_entry(PathBuf::from("path/to/another/document.txt"), 2048);
    /// assert_eq!(block.size(), 3072);
    /// ```
    pub fn add_entry(&mut self, path: PathBuf, size: u64) {
        self.entries.push(path);
        self.size += size;
    }
}

impl IntoIterator for Block {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}


/// A collection of blocks of documents.
///
/// This struct is used to manage multiple blocks of documents, where each block
/// contains a group of documents and their total size does not exceed a specified
/// maximum block size.
pub struct DocumentBlocks {
    blocks: Vec<Block>,
    max_block_size: usize,
}

impl DocumentBlocks {
    /// Creates a `DocumentBlocks` instance from a vector of directory entries.
    ///
    /// This function processes the given entries, grouping them into blocks so that
    /// the total size of each block does not exceed the specified maximum block size.
    /// The entries are processed in reverse order, and each entry is added to a block
    /// if the total size of the block plus the size of the entry is less than the
    /// maximum block size. Entries that are added to a block are removed from the
    /// original entries vector.
    ///
    /// # Arguments
    ///
    /// * `entries` - A vector of `DirEntry` objects representing the documents.
    /// * `max_block_size` - The maximum size of each block in bytes.
    ///
    /// # Returns
    ///
    /// * `io::Result<Self>` - A result containing the `DocumentBlocks` if successful,
    ///   or an `io::Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an `io::Error` if there is an error retrieving the
    /// metadata for an entry.
    pub fn from_entries(entries: Vec<DirEntry>, max_block_size: usize) -> io::Result<Self> {
        let mut entries = entries;
        let mut blocks = Vec::new();
        while !entries.is_empty() {
            let block = extract_block(&mut entries, max_block_size)?;
            blocks.push(block);
        }
        Ok(Self { blocks, max_block_size })
    }
}

/// Extracts a block of documents from the given entries, ensuring the total size of the
/// block does not exceed the specified maximum block size.
///
/// This function iterates over the entries in reverse order, adding each entry to the
/// block if the total size of the block plus the size of the entry is less than the
/// maximum block size. Entries that are added to the block are removed from the original
/// entries vector.
///
/// # Arguments
///
/// * `entries` - A mutable reference to a vector of `DirEntry` objects representing the
///    documents.
/// * `max_block_size` - The maximum size of the block in bytes.
///
/// # Returns
///
/// * `io::Result<Block>` - A result containing the created `Block` if successful, or an
///   `io::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an `io::Error` if there is an error retrieving the metadata
/// for an entry.
fn extract_block(entries: &mut Vec<DirEntry>, max_block_size: usize) -> io::Result<Block> {
    let mut block = Block::new();
    for idx in (0..entries.len()).rev() {
        let entry_size = entries[idx].metadata()?.len();
        if block.size + entry_size < max_block_size as u64 {
            block.add_entry(entries[idx].path(), entry_size);
            entries.remove(idx);
        }
    }
    Ok(block)
}

impl IntoIterator for DocumentBlocks {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.into_iter()
    }
}
