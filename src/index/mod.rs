use crate::tokenize::*;
use std::collections::HashMap;

/// An in-memory index.
///
/// This index stores the indexed document(s) in memory. For large corpus of document(s)
/// the index might not fit in-memory, and you might need to use an on-disk index.
pub struct InMemoryIndex {
    /// The total number of words in the indexed document(s).
    count: usize,

    map: HashMap<String, Vec<Hit>>
}

/// A `Hit` indicates
pub type Hit = (usize, usize, usize);

